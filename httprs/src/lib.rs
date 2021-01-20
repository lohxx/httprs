#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

pub mod url;
pub mod request;
pub mod connection;
pub mod headers;
pub mod methods;
pub mod response;

use url::URL;
use request::Request;
use methods::Method;
use headers::Header;
use connection::Connection;
use response::Response;


#[cfg(test)]
mod tests {
    use super::Request;
    
    #[test]
    fn test_head_request() {
        let response = Request::get("https://rickandmortyapi.com/api/character/21", Some(vec![]));
        assert_eq!(response.status_code, String::from("200"));
    }
}
