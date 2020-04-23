
use gaia_socket::{ClientSocket, ClientSocketImpl};
use crate::internal_shared::find_ip_address;

const DEFAULT_PORT: &str = "3179";

pub struct Client {
    socket: ClientSocketImpl
}

impl Client {
    pub fn new() -> Client {

        let mut client_socket = ClientSocketImpl::new();

        client_socket.on_connection(|sender| {
            println!("Client on_connection()");
        });

        client_socket.on_receive(|sender, msg| {
            println!("Client on_receive(): {:?}", msg);
        });

        client_socket.on_disconnection(|| {
            println!("Client on_disconnection()");
        });

        let current_socket_address = find_ip_address::get() + ":" + DEFAULT_PORT;
        client_socket.connect(current_socket_address.as_str());

        client_socket.send("just one extra post-connect message...");

        Client {
            socket: client_socket
        }
    }

    pub fn update(&mut self) {
        self.socket.update();
    }

    pub fn on_connect(&mut self, func: fn()) {

    }

    pub fn on_disconnect(&mut self, func: fn()) {

    }

    pub fn connect(&mut self) {

    }

    pub fn queue_message(&mut self) {

    }

    pub fn receive(&mut self) {

    }
}
