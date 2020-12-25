use std::fmt::Debug;

const HTTP_PORT: &str = "80";
const HTTPS_PORT: &str = "443";
const HTTP_VERSION: &str = "HTTP/1.1";

#[derive(Debug)]
pub struct Request {
    pub hostname: String,
    pub path: String,
    pub query_params: Option<String>,
    pub headers: Option<String>,
    pub method: Option<String>,
    pub server_address: String
}

pub struct Response {}

impl Request {
    pub fn new(uri: &str) -> Self {
        let (host, port) = Request::scheme_port(&uri);

        let (hostname_without_path, path_params) = match host.find('/') {
            Some(byte_index) => (&host[..byte_index], &host[byte_index..]),
            None => (host, "/")
        };

        Self {
            method: None,
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

    pub fn get(uri: &str, connection: Option<String>) -> Self {
        let request = Request::new(uri);

        return request;

    }
}