use std::{
    error::Error, io::{prelude::*, BufReader}, net::{SocketAddr, TcpListener, TcpStream}};

use chrono::{DateTime, Local};

// TODO Continue tutorial
// https://doc.rust-lang.org/book/ch20-01-single-threaded.html

/// Get a timestamp
///
fn timestamp() -> String {
    let time: DateTime<Local> = Local::now();
    return format!("{}", time.format("%Y-%m-%d %H:%M:%S%.6f"));
}


// https://danielkeep.github.io/practical-intro-to-macros.html
// https://veykril.github.io/tlborm/decl-macros/macros-methodical.html

#[macro_export]
macro_rules! info {
    // Match: a repeating sequence $();
    // matching one or more times, separated by a comma: $(),*
    // which repeats an expression, captured as a variable "arg":
    // ($arg:expr)
    // -> ( $($arg:expr),+ )
    
    ( $($arg:expr),+  ) => { println!("[{}] {}", timestamp(), format_args!($($arg),+) ) }
}

/// Read the stream until an empty line.
///
fn get_request(stream: TcpStream) -> Vec<String> {
    let reader = BufReader::new(&stream);    
    return reader.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
}


fn main() -> Result<(), Box<dyn Error>> {    

    // Address to listen to
    let backend = SocketAddr::from(([127,0,0,1], 8993));
    info!("Listening to {backend}");

    // Open connection
    let listener = TcpListener::bind(&backend)?;

    // Read the incoming data
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("{:#?}", get_request(stream));
            },
            Err(e) => {
                info!("Something went wrong: {e:?}");
            }
        }
    }
    

    Ok(())
}
