#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

mod url;
mod request;
mod connection;
mod headers;
mod methods;

use crate::url::URL;
use request::Request;
use methods::Method;
use headers::Headers;
use connection::Connection;

use std::env;

fn main() -> std::io::Result<()> {
    let mut args = env::args().skip(1);
    let uri = args.next()
        .expect("É necessario fornecer um endereço");

    let method = args.next();

    if method == Some(String::from("get")) {
        let get_response = Request::get(
            &uri,
            Some(vec![])
        );
        println!("{}", get_response);
    }

    if method == Some(String::from("post")) {
        let response = Request::post(
            &uri,
            Some(""),//Some(r#"{"email": "lohanna.dev@gmail.com", "password": "123456789"}"#),
            Some(vec![("content-type", "application/json")])
        );

        println!("{}", response);
    }

    if method == Some(String::from("head")) {
        let head_response = Request::head(&uri, Some(vec![]));
        println!("{}", head_response);
    }

    if method == Some(String::from("put")) {
        let put_response = Request::put(
            &uri,
            Some(r#"{"id": 1, "title": "teste_post"}"#),
            Some(vec![("content-type", "application/json")])
        );
        println!("{}", put_response);
    }



    Ok(())
}
