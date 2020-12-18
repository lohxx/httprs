mod request;

use request::Request;

use std::env;


fn main() -> std::io::Result<()> {
    let mut args = env::args().skip(1);
    let uri = args.next().expect("É necessario fornecer um endereço");

    let request = Request::new(&uri);

    let response = request.get();

    dbg!(request);
    // 

    // let address = format!("{}:{}", hostname_without_path, port);
    
    // println!("{}", request);
    
    // let mut res = vec![];
    // stream.read_to_end(&mut res).unwrap();
    // println!("{}", String::from_utf8_lossy(&res));

    Ok(())
}
