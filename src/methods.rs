use std::fmt::Debug;
use std::fmt::Display;

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


impl Method {
    pub fn should_set_content_length(&self) -> bool {
        match self {
            Method::PUT | Method::POST | Method::PATCH => true,
            _ => false
        }
    }
}