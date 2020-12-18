extern crate rustls;
extern crate native_tls;

use std::io;
use io::Read;
use io::Write;
use native_tls::TlsConnector;

use rustls::*;
use webpki;
use webpki_roots;

use std::env;
use std::net::TcpStream;


const HTTP_PORT: &str = "80";
const HTTPS_PORT: &str = "443";
const HTTP_VERSION: &str = "HTTP/1.1";

fn validate_scheme(hostname: &str) -> (&str, &str) {
    match &hostname[..5] {
        "http:" => (&hostname[7..], HTTP_PORT),
        "https" => (&hostname[8..], HTTPS_PORT),
        _ => (hostname, HTTP_PORT)
    }
}

fn secure_connection(hostname: &str) -> rustls::ClientSession {
    let mut config = rustls::ClientConfig::new();
    config
        .root_store
        .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
    let arc = std::sync::Arc::new(config);
    rustls::ClientSession::new(
        &arc, webpki::DNSNameRef::try_from_ascii_str(hostname).unwrap())
}

fn main() -> std::io::Result<()> {
    let mut _buffer = [0; 2024];
    let mut args = env::args().skip(1);
    let uri = args.next()
        .expect("É necessario fornecer um endereço");

    let connector = TlsConnector::new().unwrap();
    let (uri, port) = validate_scheme(&uri);

    let (hostname_without_path, path_params) = match uri.find('/') {
        Some(byte_index) => (&uri[..byte_index], &uri[byte_index..]),
        None => (uri, "/")
    };

    let address = format!("{}:{}", hostname_without_path, port);
    let socket = std::net::TcpStream::connect(&address.as_str())
        .expect("Não conseguiu se conectar no socket");
    
    let mut stream = connector.connect(hostname_without_path, socket).unwrap();

    
    let request = format!("GET {} HTTP/1.1\r\nConnection: close\r\nUser-Agent: teste\r\nAccept: */*\r\nHost: {}\r\n\r\n", path_params, hostname_without_path);
    stream.write_all(request.as_bytes()).unwrap();

    println!("{}", request);
    
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));

    Ok(())
}
