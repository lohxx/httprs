use std::fmt::Debug;
use std::fmt::Display;
use std::collections::HashMap;

use super::Connection;
use super::Header;
use super::Method;
use super::URL;
use super::Response;


const HTTP_PORT: &str = "80";
const HTTPS_PORT: &str = "443";
const HTTP_VERSION: &str = "HTTP/1.1";

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct QueryParam {
    key: String,
    value: String
}

impl Display for QueryParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

impl QueryParam {
    fn new(k: String, v: String) -> Self {
        Self {
            key: k,
            value: v,
        }
    }
}

#[derive(Debug)]
pub struct Request<'a> {
    pub url: URL<'a>,
    pub method: Method,
    pub headers: Vec<Header<'a>>,
    pub body: &'static str,
    pub query_params: Option<Vec<QueryParam>>,
    wants_secure_connection: bool
}

// TODO: Lidar com compressão, conexões persistentes e adicionar gerenciamento de erros.

impl Display for Request<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let textual_headers: String = self.headers
            .iter()
            .map(|h| h.to_string())
            .collect();

        let path = match &self.query_params {
            Some(q_params) => {
                let q: String = q_params
                    .iter()
                    .map(|q| q.to_string())
                    .collect();

                format!("{}?{}", self.url.path, q)
            },
            None => format!("{}", self.url.path)
        };

        write!(
            f, "{} {} {}\r\n{}\r\n{}",
            self.method,
            path,
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
        headers: Option<Vec<(&'a str, &'a str)>>,
        query_params: Option<Vec<(String, String)>>
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

        if headers.is_some() {
            for header in headers.unwrap() {
                &mapped_headers.push(Header::from(header));
            }
        }

        let mut qparams = vec![];

        if query_params.is_some() {
            for query_param in query_params.unwrap() {
                qparams.push(QueryParam::new(query_param.0, query_param.1));
            }
        }

        Self {
            url,
            method,
            headers: mapped_headers,
            body: body.unwrap_or(""),
            wants_secure_connection: secure_connection,
            query_params: match qparams.len() >= 1 {
                true => Some(qparams),
                false => None
            },
            
        }
    }

    fn raw_request(
        uri: &str,
        method: Method,
        data: &'static str,
        headers: Option<Vec<(&str, &str)>>,
        query_params: Option<Vec<(String, String)>>) -> String {

        let request = Request::new(
            uri, method, Some(data), headers, query_params);
            
        println!("{}", request);

        let bytes = Connection::new(
            request.url.hostname,
            request.wants_secure_connection,
            request.url.server_address().as_str()
        ).send(request.to_string());
 
        String::from_utf8_lossy(&bytes).to_string()
    }

    pub fn get(
        uri: &str,
        headers: Option<Vec<(&str, &str)>>,
        query_params: Option<Vec<(String, String)>>) -> Response {
        let mut response = Request::raw_request(uri, Method::GET, "", headers, query_params);
        Response::parse(response)
    }

    pub fn head(
        uri: &str,
        headers: Option<Vec<(&str, &str)>>,
        query_params: Option<Vec<(String, String)>>) -> Response {
        let txt_response = Request::raw_request(uri, Method::HEAD, "", headers, query_params);
        Response::parse(txt_response)
    }

    pub fn post(
        uri: &str,
        data: Option<&'static str>,
        headers: Option<Vec<(&str, &str)>>,
        query_params: Option<Vec<(String, String)>>) -> Response {

        let len: &str = &data.unwrap_or("").len().to_string();

        let mut extra_headers = vec![("Content-Length", len)];

        extra_headers.append(&mut headers.unwrap());

        let txt_response = Request::raw_request(
            uri, Method::POST, data.unwrap(), Some(extra_headers), query_params);
        
        Response::parse(txt_response)
    }

    pub fn put(
        uri: &str,
        data: Option<&'static str>,
        headers: Option<Vec<(&str, &str)>>,
        query_params: Option<Vec<(String, String)>>) -> Response {
        
        let len: &str = &data.unwrap_or("").len().to_string();

        let mut extra_headers = vec![("Content-Length", len)];

        extra_headers.append(&mut headers.unwrap());

        let txt_response = Request::raw_request(
            uri, Method::PUT, data.unwrap(), Some(extra_headers), query_params);
        
        Response::parse(txt_response)
    }
}
