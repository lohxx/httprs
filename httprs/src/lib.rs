#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

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
mod test_request {
    use super::Request;
    
    #[test]
    fn get() {
        let response = Request::get("https://rickandmortyapi.com/api/character/21", None, None);
        assert_eq!(response.status_code, 200);
        assert_ne!(response.body, None);
        assert_eq!(response.phrase, "OK");
        assert!(response.headers.keys().len() >= 1);
    }

    #[test]
    fn head() {
        let response = Request::head("https://http.cat/201", None, None);
        assert_eq!(response.body, None);
        assert_eq!(response.status_code, 200);
    }

    #[test]
    fn post() {}

    fn put(){}
}
