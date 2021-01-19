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
use headers::Headers;
use connection::Connection;
use response::Response;


#[cfg(test)]
mod tests {
    use super::Request;
    
    #[test]
    fn test_head_request() {
        let response = Request::head("https://github.com/lohxx", Some(vec![]));
        //dbg!(response);
    }
}
