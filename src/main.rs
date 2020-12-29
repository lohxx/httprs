#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

mod request;
mod connection;
mod headers;
mod methods;

use request::Request;
use methods::Method;
use headers::Headers;
use connection::Connection;

use std::env;

fn main() -> std::io::Result<()> {
    let mut args = env::args().skip(1);
    let uri = args.next()
        .expect("É necessario fornecer um endereço");

    let response = Request::post(
        &uri,
        Some(r#"{"id": 776, "title": "teste_post"}"#),
        Some(vec![("content-type", "application/json")])
    );

    println!("{}", response);

    let get_response = Request::get(&uri, Some(vec![]));

    println!("{}", get_response);


    let head_response = Request::head(&uri, Some(vec![]));

    println!("{}", head_response);

    Ok(())
}
