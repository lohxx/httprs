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

use serde_json::{Result, Value};


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
        assert_eq!(response.phrase, "OK");
    }

    #[test]
    fn post() {
        // ("Connection", "keep-alive")
        let data = Some(r#"{"title": "foo", "body": "bar", "userId": 3}"#);
        let headers = Some(vec![("Content-Type", "application/json")]);
        let response = Request::post(
            "https://jsonplaceholder.typicode.com/posts", data, headers, None);
        assert_eq!(response.status_code, 201);
        assert_eq!(response.phrase, "Created");
    }

    #[test]
    fn put() {
        let data = Some(r#"{"title": "foo", "body": "bar", "userId": 3}"#);
        let headers = Some(vec![("Content-Type", "application/json")]);
        let response = Request::put(
            "https://jsonplaceholder.typicode.com/posts/1", data, headers, None);
        assert_eq!(response.status_code, 200);
        assert_eq!(response.phrase, "OK");
        dbg!(response);

        //serde_json::from_str(response.body.unwrap().as_str()).unwrap();

    }
}
