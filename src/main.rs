#![allow(dead_code)]

use std::{
    fmt,
    error::Error,
    fs,
    io::{BufReader, BufRead, Write},
    net::{SocketAddr, TcpListener, TcpStream}
};

use regex::Regex;

// It seems like we need to import the 'private' helper function
// timestamp() so that it shares the macro's scope. This is weird, and
// it shows that Rust would profit from a way to pass variadic
// arguments to normal functions. There is no other reason to use a
// macro here.
mod log;
use log::timestamp;

mod status;

type URI = String;

// TODO ServerError should be a reply / response
#[derive(Debug)]
enum ServerError {
    BadRequest,
    InvalidHeader,
    Unknown,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ServerError::Unknown => write!(f, "Something went wrong"),
            ServerError::BadRequest => write!(f, "Incomplete request"),
            ServerError::InvalidHeader => write!(f, "Could not parse request header line(s)"),
        }
    }
}

#[derive(Debug)]
enum HTTPMethod {
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
    status_code: status::StatusCode,
    headers: Vec<String>,
    body: String   
}

impl Response {

    // Internal "private" method
    fn join(&self) -> String {
        let response = self;
        let content_header = format!("Content-Length: {}\r\n", response.body.len());
        return format!("{} {}\r\n{}\r\n{content_header}\r\n{}\r\n",
            response.protocol, response.status_code,
            response.headers.join("\r\n"),
            response.body);
    }  

    // Public method
    fn send(&self, mut stream: TcpStream) -> Result<(), ServerError> {
        match stream.write_all(self.join().as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServerError::Unknown),
        }
    }
}


/// Read the stream and parse it as a request.
///
fn get_request(stream: &TcpStream) -> Result<Request, ServerError>  {
    let reader = BufReader::new(stream);    

    let raw_request : Result<Vec<String>, _> = reader.lines()
        .take_while(|line| line.is_ok() && !line.as_ref().unwrap().is_empty())
        .collect();

    return if raw_request.is_err() {
         Err(ServerError::BadRequest)
    } else {
        to_request(raw_request.unwrap().as_mut())
    };
}

/// Transform raw header data to a typed request
///
fn to_request(raw_headers: &mut Vec<String>) -> Result<Request, ServerError> {
    if raw_headers.len() < 1 {
        return Err(ServerError::BadRequest);
    } else {
        return identify(&raw_headers[0])
            .map(|(_method, _uri)| Request{ method: _method, uri: _uri, header: raw_headers.split_off(1)});
    }
}

/// Find out the request type
fn identify(first_line: &str) -> Result<(HTTPMethod, String), ServerError> {
    // TODO Also scan URL parameters (?foo=x&bar=z)
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
                "GET"  => HTTPMethod::GET,
                _ => HTTPMethod::UNKNOWN,
            };

            let uri = groups.name("uri").unwrap().as_str();

            return Ok((method, String::from(uri)));            
        },
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
                    status_code: status::StatusCode::OK,
                    headers: vec!["".to_string()],
                    body: file,
                };

                // TODO Somehow make "?" possible ("From"-Trait?)
                let _ = response.send(stream);
            },
            Err(e) => {
                info!("Something went wrong: {e:?}");
            }
        }
    }
    
    Ok(())
}
