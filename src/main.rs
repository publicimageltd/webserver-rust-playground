use core::fmt;
use std::{
    borrow::BorrowMut, error::Error, fs, io::{prelude::*, BufReader, BufWriter}, net::{SocketAddr, TcpListener, TcpStream}};

use chrono::{DateTime, Local};
use regex::Regex;

// TODO Continue tutorial
// https://doc.rust-lang.org/book/ch20-01-single-threaded.html


type URI = String;

#[derive(Debug)]
enum ServerError {
    IncompleteRequest,
    InvalidHeader,
    Unknown,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ServerError::Unknown => write!(f, "Something went wrong"),
            ServerError::IncompleteRequest => write!(f, "Incomplete request"),
            ServerError::InvalidHeader => write!(f, "Could not parse header line(s)"),
        }
    }
}

#[derive(Debug)]
enum HTTPMethod {
    POST,
    GET,
    UNKNOWN,
}


#[derive(Debug)]
struct Request {
    method: HTTPMethod,
    uri: URI,
    header: Vec<String>,
}

#[derive(Debug)]
struct Response {
    protocol: String,
    status_code: String,
    reason: String,
    headers: Vec<String>,
    body: String   
}

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

/// Read the stream and parse it as a request.
///
fn get_request(stream: &TcpStream) -> Result<Request, ServerError>  {
    let reader = BufReader::new(stream);    

    let raw_request : Result<Vec<String>, _> = reader.lines()
        .take_while(|line| line.is_ok() && !line.as_ref().unwrap().is_empty())
        .collect();

    return if raw_request.is_err() {
         Err(ServerError::IncompleteRequest)
    } else {
        to_request(raw_request.unwrap().as_mut())
    };
}

/// Transform raw header data to a typed request
///
fn to_request(raw_headers: &mut Vec<String>) -> Result<Request, ServerError> {
    if raw_headers.len() < 1 {
        return Err(ServerError::IncompleteRequest);
    } else {
        return identify(&raw_headers[0])
            .map(|(_method, _uri)| Request{ method: _method, uri: _uri, header: raw_headers.split_off(1)});                    
    }
}

/// Find out the request type
fn identify(first_line: &str) -> Result<(HTTPMethod, String), ServerError> {
    let re = Regex::new(r"(?<method>[A-Z]+) (?<uri>/\S*) (?<protocol>\S+)").unwrap();
    match re.captures(first_line) {
        None => Err(ServerError::InvalidHeader),
        Some(groups) => {
            
            if groups.name("method").is_none()
                || groups.name("uri").is_none()
                || groups.name("protocol").is_none()  {
                    return Err(ServerError::InvalidHeader);
                };
                        
            let method  = groups.name("method").unwrap().as_str();
            let method = match method {
                "POST" => HTTPMethod::POST,
                "GET"  => HTTPMethod::GET,
                _ => HTTPMethod::UNKNOWN,
            };

            let uri = groups.name("uri").unwrap().as_str();

            return Ok((method, String::from(uri)));            
        },
    }
} 

fn join_response(response: &Response) -> String {
    let content_header = format!("Content-Length: {}\r\n", response.body.len());
    return format!("{} {} {}\r\n{}\r\n{content_header}\r\n{}\r\n",
        response.protocol, response.status_code, response.reason,
        response.headers.join("\r\n"),
        response.body);
}

fn send_response(response: &Response, mut stream: TcpStream) -> Result<(), ServerError> {
    match stream.write_all(join_response(response).as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServerError::Unknown),
    }
}

/// Listen and reply
fn main() -> Result<(), Box<dyn Error>> { 

    // Address to listen to
    let backend = SocketAddr::from(([127,0,0,1], 8993));
    info!("Listening to {backend}");

    // Open connection
    let listener = TcpListener::bind(&backend)?;

    // Read the incoming data and respond
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("{:#?}", get_request(&stream));

                let file = fs::read_to_string("hello.html").unwrap_or("<p>FEHLER BEIM EINLESEN DER DATEI!</p>".to_string());
                
                let response = Response {
                    protocol: "HTTP/1.1".to_string(),
                    status_code: "200".to_string(),
                    reason: "OK".to_string(),
                    headers: vec!["".to_string()],
                    body: file,
                };

                // TODO Somehow make "?" possible ("From"-Trait?)
                let _ = send_response(&response, stream);                
            },
            Err(e) => {
                info!("Something went wrong: {e:?}");
            }
        }
    }
    
    Ok(())
}
