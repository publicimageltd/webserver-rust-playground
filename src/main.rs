use std::{
    error::Error,
    net::{self, SocketAddr, TcpListener, TcpStream},
    io::{prelude::*, BufReader}};

// TODO Add timestamp to output
// TODO Continue tutorial
// https://doc.rust-lang.org/book/ch20-01-single-threaded.html

fn get_request(mut stream: TcpStream) -> Vec<String> {
    let reader = BufReader::new(&stream);    
    return reader.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
}

fn main() -> Result<(), Box<dyn Error>> {    

    // Address to listen to
    let backend = SocketAddr::from(([127,0,0,1], 8993));

    // Open connection
    let listener = TcpListener::bind(&backend)?;

    // Read the incoming data
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established! {stream:?}");
                println!("{:#?}", get_request(stream));
            },
            Err(e) => {
                println!("Something went wrong: {e:?}");
            }
        }
    }
    

    Ok(())
}
