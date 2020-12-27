#![allow(unused_imports)]
#![allow(dead_code)]



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

    Ok(())
}
