extern crate log;

use std::{
    io::ErrorKind,
    net::{SocketAddr, UdpSocket},
};

use naia_socket_shared::{find_my_ip_address, LinkConditionerConfig, Ref};

use crate::{link_conditioner::LinkConditioner, ClientSocketTrait, MessageSender};

use crate::{error::NaiaClientSocketError, ClientSocketConfig, Packet};

/// A client-side socket which communicates with an underlying unordered &
/// unreliable protocol
#[derive(Debug)]
pub struct ClientSocket {
    address: SocketAddr,
    socket: Ref<UdpSocket>,
    receive_buffer: Vec<u8>,
    message_sender: MessageSender,
}

impl ClientSocket {
    /// Returns a new ClientSocket, connected to the given socket address
    pub fn connect(client_config: ClientSocketConfig) -> Box<dyn ClientSocketTrait> {
        let client_ip_address = find_my_ip_address().expect("cannot find current ip address");

        let socket = Ref::new(UdpSocket::bind((client_ip_address, 0)).unwrap());
        socket
            .borrow()
            .set_nonblocking(true)
            .expect("can't set socket to non-blocking!");

        let message_sender = MessageSender::new(client_config.server_address, socket.clone());

        let mut client_socket: Box<dyn ClientSocketTrait> = Box::new(ClientSocket {
            address: client_config.server_address,
            socket,
            receive_buffer: vec![0; 1472],
            message_sender,
        });

        if let Some(config) = &client_config.shared.link_condition_config {
            client_socket = client_socket.with_link_conditioner(config);
        }

        client_socket
    }
}

impl ClientSocketTrait for ClientSocket {
    fn receive(&mut self) -> Result<Option<Packet>, NaiaClientSocketError> {
        let buffer: &mut [u8] = self.receive_buffer.as_mut();
        match self
            .socket
            .borrow()
            .recv_from(buffer)
            .map(move |(recv_len, address)| (&buffer[..recv_len], address))
        {
            Ok((payload, address)) => {
                if address == self.address {
                    return Ok(Some(Packet::new(payload.to_vec())));
                } else {
                    return Err(NaiaClientSocketError::Message(
                        "Unknown sender.".to_string(),
                    ));
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                //just didn't receive anything this time
                return Ok(None);
            }
            Err(e) => {
                return Err(NaiaClientSocketError::Wrapped(Box::new(e)));
            }
        }
    }

    fn get_sender(&mut self) -> MessageSender {
        return self.message_sender.clone();
    }

    fn with_link_conditioner(
        self: Box<Self>,
        config: &LinkConditionerConfig,
    ) -> Box<dyn ClientSocketTrait> {
        Box::new(LinkConditioner::new(config, self))
    }
}
