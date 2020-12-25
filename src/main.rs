mod request;
mod connection;

use request::Request;
use connection::Connection;

use std::env;

fn main() -> std::io::Result<()> {
    let mut args = env::args().skip(1);
    let uri = args.next().expect("É necessario fornecer um endereço");

    let request = Request::get(&uri, None);

    dbg!(request);

    let stream = Connection::new(&request.hostname.as_str(), true);
    
    let request = format!(
        "GET {} HTTP/1.1\r\nConnection: close\r\nUser-Agent: teste\r\nAccept: */*\r\nHost: {}\r\n\r\n",
        request.path,
        request.hostname);
    
    println!("{}", request);
    let bytes = stream.send(request);
    println!("{}", String::from_utf8_lossy(&bytes));

    Ok(())
}
