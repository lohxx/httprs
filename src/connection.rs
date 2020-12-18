extern crate rustls;
extern crate native_tls;

use std::io;
use io::Read;
use io::Write;
use native_tls::TlsConnector;

use rustls::*;
use webpki;
use webpki_roots;

use std::net::TcpStream;

fn tls_client_session(hostname: &str) -> rustls::ClientSession {
    let mut config = rustls::ClientConfig::new();
    config
        .root_store
        .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

    rustls::ClientSession::new(
        std::sync::Arc::new(config), 
        webpki::DNSNameRef::try_from_ascii_str(hostname).unwrap())
}

fn send(server: ) {
    let connector = TlsConnector::new().unwrap();
    let socket = std::net::TcpStream::connect(&address.as_str())
        .expect("Não conseguiu se conectar no socket");
    
    let mut stream = connector.connect(hostname_without_path, socket).unwrap();

    let request = format!("GET {} HTTP/1.1\r\nConnection: close\r\nUser-Agent: teste\r\nAccept: */*\r\nHost: {}\r\n\r\n", path_params, hostname_without_path);
    stream.write_all(request.as_bytes()).unwrap();

}