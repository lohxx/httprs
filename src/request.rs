use std::fmt::Debug;
use std::fmt::Display;

use super::Connection;

const HTTP_PORT: &str = "80";
const HTTPS_PORT: &str = "443";
const HTTP_VERSION: &str = "HTTP/1.1";

#[derive(Debug)]
pub struct Request {
    pub hostname: String,
    pub path: String,
    pub query_params: Option<String>,
    pub headers: Option<String>,
    pub method: Method,
    pub server_address: String
}

#[derive(Debug)]
pub enum Method {
    GET,
    HEAD,
    POST,
    DELETE,
    PUT,
    PATCH,
    OPTIONS,
    TRACE
}

impl Display for Method {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            Method::GET => f.write_str("GET"),
            Method::HEAD => f.write_str("HEAD"),
            Method::POST => f.write_str("POST"),
            Method::DELETE => f.write_str("DELETE"),
            Method::OPTIONS => f.write_str("OPTIONS"),
            Method::PATCH => f.write_str("PATCH"),
            Method::PUT => f.write_str("PUT"),
            Method::TRACE => f.write_str("TRACE")
        }
    }
}


impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}\r\nConnection: close\r\nHost: {}\r\nAccept: */*\r\n\r\n", self.method, self.path, HTTP_VERSION, self.hostname)
    }
}

impl Request {
    pub fn new(uri: &str, method: Option<Method>) -> Self {
        let (host, port) = Request::scheme_port(&uri);

        let (hostname_without_path, path_params) = match host.find('/') {
            Some(byte_index) => (&host[..byte_index], &host[byte_index..]),
            None => (host, "/")
        };

        Self {
            method: method.unwrap(),
            headers: None,
            query_params: None,
            path: String::from(path_params),
            hostname: String::from(hostname_without_path),
            server_address: format!("{}:{}", hostname_without_path, port),
        }
    }

    fn scheme_port(hostname: &str) -> (&str, &str) {
        match &hostname[..5] {
            "http:" => (&hostname[7..], "80"),
            "https" => (&hostname[8..], "443"),
            _ => (&hostname, "80")
        }
    }

    pub fn get(uri: &str) -> std::borrow::Cow<str>{
        let request = Request::new(uri, Some(Method::GET));
        
        let stream = Connection::new(&request.hostname, true);

        let bytes = stream.send(request.to_string());

        String::from_utf8_lossy(&bytes)
    }
}