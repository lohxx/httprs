use std::fmt::Debug;
use std::fmt::Display;

use super::Connection;
use super::Headers;
use super::Method;
use super::URL;


const HTTP_PORT: &str = "80";
const HTTPS_PORT: &str = "443";
const HTTP_VERSION: &str = "HTTP/1.1";


#[derive(Debug)]
pub struct Request<'a> {
    pub url: URL<'a>,
    pub method: Method,
    pub headers: Headers<'a>,
    pub body: &'static str,
    wants_secure_connection: bool
}

// TODO: Lidar com compressão e conexões persistentes.


impl Display for Request<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{} {} {}\r\n{}\r\n{}\r\n",
            self.method,
            self.url.path,
            HTTP_VERSION,
            self.headers,
            self.body
        )
    }
}

impl <'a>Request<'a> {
    pub fn new(
        uri: &'a str,
        method: Method,
        body: Option<&'static str>,
        headers: Option<Vec<(&'a str, &'a str)>>
    ) -> Self {

        let url = URL::parse(uri);

        let mut secure_connection = true; 

        if url.port == HTTP_PORT {
            secure_connection = false;
        }
 
        let mut mapped_headers = Headers::new(vec![
            ("Accept", "*/*"),
            ("Connection", "close"),
            ("Host", url.hostname),
        ]);

        for header in headers.unwrap() {
            &mapped_headers.insert(header);
        }

        Self {
            url,
            method,
            headers: mapped_headers,
            body: body.unwrap_or(""),
            wants_secure_connection: secure_connection,
        }
    }

    fn raw_request(
        uri: &str,
        method: Method,
        data: &'static str,
        headers: Option<Vec<(&str, &str)>>) -> String {

        let request = Request::new(
            uri, method, Some(data), headers);

        dbg!(&request);

        let bytes = Connection::new(
            request.url.hostname,
            request.wants_secure_connection,
            request.url.server_address().as_str()
        ).send(request.to_string());

        String::from_utf8_lossy(&bytes).to_string()
    }

    pub fn get(uri: &str, headers: Option<Vec<(&str, &str)>>) -> String {
        Request::raw_request(uri, Method::GET, "", headers)   
    }

    pub fn head(uri: &str, headers: Option<Vec<(&str, &str)>>) -> String {
        Request::raw_request(uri, Method::HEAD, "", headers)
    }

    pub fn post(
        uri: &str,
        data: Option<&'static str>,
        headers: Option<Vec<(&str, &str)>>) -> String {

        let len: &str = &data.unwrap_or("").len().to_string();

        let mut extra_headers = vec![("Content-Length", len)];

        extra_headers.append(&mut headers.unwrap());

        Request::raw_request(uri, Method::POST, data.unwrap(), Some(extra_headers))
    }

    pub fn put(
        uri: &str,
        data: Option<&'static str>,
        headers: Option<Vec<(&str, &str)>>) -> String {
        
        let len: &str = &data.unwrap_or("").len().to_string();

        let mut extra_headers = vec![("Content-Length", len)];

        extra_headers.append(&mut headers.unwrap());
        
        Request::raw_request(uri, Method::PUT, data.unwrap(), Some(extra_headers))
    }
}