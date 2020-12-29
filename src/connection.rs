extern crate rustls;
extern crate native_tls;

use std::io;
use io::Read;
use io::Write;
use native_tls::{TlsConnector, TlsStream};

use rustls::*;
use webpki;
use webpki_roots;

use std::fmt::Debug;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Connection {
    address: String,
    tcp_socket: TcpStream,
    wants_secure_connection: bool, 
    tls_connector: TlsConnector
}

impl Connection {
    pub fn new(name: &str, secure_connection: bool, server_address: &str) -> Self {
        let socket = TcpStream::connect(server_address)
            .expect("Não foi possivel se conectar no socket");

        Self {
            address: name.to_string(),
            tcp_socket: socket,
            tls_connector: TlsConnector::new().unwrap(),
            wants_secure_connection: secure_connection
        }
    }

    /// Envia a requisição para o servidor usando um meio criptografado.
    fn send_via_secure_connection(&self, request: &str) -> Vec<u8> {
        let mut response = vec![];

        let mut stream = self.tls_connector.connect(
            self.address.as_str(),
            &self.tcp_socket
        ).unwrap();

        stream.write_all(request.as_bytes());

        stream.read_to_end(&mut response);

        return response;
    }

    pub fn send(&self, request: String) -> Vec<u8> {     
        if self.wants_secure_connection {
            return self.send_via_secure_connection(request.as_str());            
        }

        // Implementar envio via conexão não segura.
        vec![]
    }

}