use std::fmt::Debug;
use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use native_tls::{TlsConnector, TlsStream};


#[derive(Debug)]
pub struct Connection {
    address: String,
    tcp_socket: TcpStream,
    wants_secure_connection: bool, 
    tls_connector: TlsConnector,
}

// Precisa ser um singleton para segurar a conexão????
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
    fn send_via_secure_connection(&mut self, request: &str) -> Vec<u8> {        
        let mut response = vec![];

        let mut stream = self.tls_connector.connect(
            self.address.as_str(),
            &self.tcp_socket
        ).unwrap();

        stream.write_all(request.as_bytes());

        stream.read_to_end(&mut response);

        response
    }

    /// Envia a requisição para o servidor via meio inseguro
    fn send_via_unsecure_connection(&mut self, request: &str) -> Result<Vec<u8>, std::io::Error> {
        let mut response = vec![];

        self.tcp_socket.write(request.as_bytes())?;

        self.tcp_socket.read_to_end(&mut response)?;

        Ok(response)
    }

    pub fn send(&mut self, request: String) -> Vec<u8> {     
        if self.wants_secure_connection {
            return self.send_via_secure_connection(request.as_str());            
        }

        self.send_via_unsecure_connection(request.as_str()).unwrap_or(vec![])
    }

}