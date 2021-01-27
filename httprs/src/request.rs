use std::fmt::Debug;
use std::fmt::Display;

use super::Connection;
use super::Header;
use super::Method;
use super::URL;
use super::Response;


const HTTP_PORT: &str = "80";
const HTTPS_PORT: &str = "443";
const HTTP_VERSION: &str = "HTTP/1.1";


#[derive(Debug)]
pub struct Request<'a> {
    pub url: URL<'a>,
    pub method: Method,
    pub headers: Vec<Header<'a>>,
    pub body: &'static str,
    wants_secure_connection: bool
}

// TODO: Lidar com compressão, conexões persistentes e adicionar gerenciamento de erros.

impl Display for Request<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let textual_headers: String = self.headers
            .iter()
            .map(|h| h.to_string())
            .collect();

        write!(
            f, "{} {} {}\r\n{}\r\n{}",
            self.method,
            self.url.path,
            HTTP_VERSION,
            textual_headers,
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

        let secure_connection = match url.scheme {"http" => false, "https" => true, _ => false}; 
        
        // required headers
        let mut mapped_headers: Vec<Header> = vec![
            Header::from(("Accept", "*/*")),
            Header::from(("Connection", "close")),
            Header::from(("Host", url.hostname)),
            Header::from(("User-Agent", "httprs"))
        ];

        for header in headers.unwrap() {
            &mapped_headers.push(Header::from(header));
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

        let bytes = Connection::new(
            request.url.hostname,
            request.wants_secure_connection,
            request.url.server_address().as_str()
        ).send(request.to_string());
 
        String::from_utf8_lossy(&bytes).to_string()
    }

    pub fn get(uri: &str, headers: Option<Vec<(&str, &str)>>) -> Response<'a> {
        let response = Request::raw_request(uri, Method::GET, "", headers);
        Response::parse(response)
    }

    pub fn head(uri: &str, headers: Option<Vec<(&str, &str)>>) -> Response<'a> {
        let txt_response = Request::raw_request(uri, Method::HEAD, "", headers);
        unimplemented!()
        //Response::parse(txt_response)
    }

    pub fn post(
        uri: &str,
        data: Option<&'static str>,
        headers: Option<Vec<(&str, &str)>>) -> Response<'a> {

        let len: &str = &data.unwrap_or("").len().to_string();

        let mut extra_headers = vec![("Content-Length", len)];

        extra_headers.append(&mut headers.unwrap());

        let txt_response = Request::raw_request(uri, Method::POST, data.unwrap(), Some(extra_headers));
        
        unimplemented!()
        //Response::parse(txt_response)
    }

    pub fn put(
        uri: &str,
        data: Option<&'static str>,
        headers: Option<Vec<(&str, &str)>>) -> Response<'a> {
        
        let len: &str = &data.unwrap_or("").len().to_string();

        let mut extra_headers = vec![("Content-Length", len)];

        extra_headers.append(&mut headers.unwrap());

        let txt_response = Request::raw_request(uri, Method::PUT, data.unwrap(), Some(extra_headers));
        
        unimplemented!()
        //Response::parse(txt_response)
    }
}
