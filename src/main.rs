use std::{
    error::Error,
    net::{self, SocketAddr, TcpListener, TcpStream},
    io::{prelude::*, BufReader}};

// TODO Add timestamp to output
// TODO Continue tutorial
// https://doc.rust-lang.org/book/ch20-01-single-threaded.html

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
                let buf_reader = BufReader::new(&stream);
                let mut target_string = String::new();
                // Old-school for loop.
                // TODO Change into a more functional style
                // TODO Is there a "while let...."?
                for line in buf_reader.lines() {
                    let line = line.unwrap();
                    if line.is_empty() {
                        break;
                    }
                    target_string.push_str(&line);
                    target_string.push_str("\n");
                }
                println!("{target_string}");
            },
            Err(e) => {
                println!("Something went wrong: {e:?}");
            }
        }
    }
    

    Ok(())
}
