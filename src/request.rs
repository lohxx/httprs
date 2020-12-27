use std::fmt::Debug;
use std::fmt::Display;

use super::Connection;

const HTTP_PORT: &str = "80";
const HTTPS_PORT: &str = "443";
const HTTP_VERSION: &str = "HTTP/1.1";

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub method: Method,
    pub hostname: String,
    pub body: String,
    pub headers: Headers,
    pub query_params: Option<String>,

    server_address: String,
    wants_secure_connection: bool
}

#[derive(Debug)]
pub struct Headers {
    headers: Vec<String>
}

impl Display for Headers {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        f.write_str(self.headers.as_slice().join("\r\n").as_str())
    }
}

impl Headers {
    pub fn new(items: Vec<String>) -> Self {
       let mut headers = Self {
           headers: vec![]
       };

       for value in items {
           headers.headers.push(value);
       }

       headers
    }

    fn insert(&mut self, value: String) {
        self.headers.push(value);
    }
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
        write!(
            f, "{} {} {}\r\n{}\r\n{}\r\n",
            self.method,
            self.path,
            HTTP_VERSION,
            self.headers,
            self.body
        )
    }
}

impl Request {
    pub fn new(
        uri: &str,
        method: Option<Method>,
        body: Option<String>,
        mut headers: Option<Vec<String>>
    ) -> Self {

        let (host, port) = Request::scheme_port(&uri);

        let (hostname_without_path, path_params) = match host.find('/') {
            Some(byte_index) => (&host[..byte_index], &host[byte_index..]),
            None => (host, "/")
        };

        let mut secure_connection = true; 

        if port == HTTP_PORT {
            secure_connection = false;
        }

        let default_headers = vec![
            String::from("Connection: close"), 
            format!("Host: {}", host), 
            String::from("Accept: */*")];

        let new_headers: Vec<String> = match headers {
            Some(mut h) => {
                for new_header in default_headers.iter() {
                    h.push(new_header.clone());
                }
                h
            },
            None => default_headers
        };

        Self {
            query_params: None,
            method: method.unwrap(),
            path: String::from(path_params),
            headers: Headers::new(new_headers),
            body: body.unwrap_or(String::from("")),
            wants_secure_connection: secure_connection,
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

    fn raw_request(uri: &str, method: Method) -> String {
        let request = Request::new(
            uri, Some(method), None, None);

        let bytes = Connection::new(
            &request.hostname,
            request.wants_secure_connection,
            &request.server_address
        ).send(request.to_string());

        String::from_utf8_lossy(&bytes).to_string()
    }

    pub fn get(uri: &str) -> String {
        Request::raw_request(uri, Method::GET)   
    }

    pub fn head(uri: &str) -> String {
        Request::raw_request(uri, Method::HEAD)
    }

    pub fn post(uri: &str, data: Option<String>) {
        let extra_headers = String::from("Content-Type: application/json\r\n");
        let request = Request::new(
            uri, Some(Method::POST), None, Some(vec![extra_headers]));
    }

    pub fn patch(uri: &str, data: Option<String>) {}
}