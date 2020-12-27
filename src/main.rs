#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

mod request;
mod connection;

use request::Request;
use connection::Connection;

use std::env;

fn main() -> std::io::Result<()> {
    let mut args = env::args().skip(1);
    let uri = args.next()
        .expect("É necessario fornecer um endereço");

    let response = Request::get(&uri);

    let head_response = Request::head(&uri);

    println!("{}", head_response);
    //println!("{}", response);

    Ok(())
}
