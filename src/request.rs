use std::fmt::Debug;
use std::fmt::Display;

use super::Connection;
use super::Headers;
use super::Method;


const HTTP_PORT: &str = "80";
const HTTPS_PORT: &str = "443";
const HTTP_VERSION: &str = "HTTP/1.1";

#[derive(Debug)]
pub struct Request<'a> {
    pub path: &'a str,
    pub method: Method,
    pub headers: Headers,
    pub hostname: &'a str,
    pub body: &'static str,

    server_address: (&'a str, &'a str),
    wants_secure_connection: bool
}


impl Display for Request<'_> {
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

impl <'a>Request<'a> {
    pub fn new(
        uri: &'a str,
        method: Option<Method>,
        body: Option<&'static str>,
        headers: Option<Vec<(&str, &str)>>
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
            ("Accept", "*/*"),
            ("Connection", "close"),
            ("Host", hostname_without_path),
        ];

        let mut map_headers = Headers::new(default_headers);

        for h in headers.unwrap() {
            map_headers.insert(h);
        }

        Self {
            path: path_params,
            headers: map_headers,
            method: method.unwrap(),
            body: body.unwrap_or(""),
            hostname: hostname_without_path,
            wants_secure_connection: secure_connection,
            server_address: (hostname_without_path, port),
        }
    }

    fn scheme_port(hostname: &str) -> (&str, &str) {
        match &hostname[..5] {
            "http:" => (&hostname[7..], "80"),
            "https" => (&hostname[8..], "443"),
            _ => (&hostname, "80")
        }
    }

    fn raw_request(uri: &str, method: Method, data: &'static str, headers: Option<Vec<(&str, &str)>>) -> String {
        let request = Request::new(
            uri, Some(method), Some(data), headers);

        dbg!(&request);

        let bytes = Connection::new(
            request.hostname,
            request.wants_secure_connection,
            format!("{}:{}",request.server_address.0,request.server_address.1).as_str()
        ).send(request.to_string());

        String::from_utf8_lossy(&bytes).to_string()
    }

    pub fn get(uri: &str, headers: Option<Vec<(&str, &str)>>) -> String {
        Request::raw_request(uri, Method::GET, "", headers)   
    }

    pub fn head(uri: &str, headers: Option<Vec<(&str, &str)>>) -> String {
        Request::raw_request(uri, Method::HEAD, "", headers)
    }

    pub fn post(uri: &str, data: Option<&'static str>, headers: Option<Vec<(&str, &str)>>) -> String {
        let len: &str = &data.unwrap_or("").len().to_string();

        let mut extra_headers = vec![("Content-Length", len)];

        extra_headers.append(&mut headers.unwrap());

        Request::raw_request(uri, Method::POST, data.unwrap(), Some(extra_headers))
    }

}