#![allow(dead_code)]

///!
///! "Web Server"
///!
///! We limit ourselves to HTTP/1.1 (see
///! https://datatracker.ietf.org/doc/html/rfc2616#section-4.1)

use std::{
    collections::HashMap, error::Error, fmt, fs, io::{BufRead, BufReader, Write}, net::{SocketAddr, TcpListener, TcpStream}
};

use header::{HeaderName, PredefinedName, HeaderMap};
use regex::Regex;

mod log;
mod error;
use error::AppError;
mod status;
mod header;

type URI = String;

// TODO ServerError should be a reply / response

#[derive(Debug)]
enum HTTPMethod {
    GET,
    UNKNOWN,
}


#[derive(Debug)]
struct Request {
    method: HTTPMethod,
    uri: URI,
    headers: HeaderMap,
}


#[derive(Debug)]
struct Response {
    protocol: String,
    status_code: status::StatusCode,
    headers: HeaderMap,
    body: String   
}

impl Response {

    // Internal "private" method
    fn join(&self) -> String {

        // TODO Move this into a filter fn
        let val = self.body.len();
        let mut additional_headers = HeaderMap::new();
        additional_headers.insert(PredefinedName::ContentLength, val);
        
        return format!("{} {}\r\n{}\r\n{}\r\n",
            self.protocol, self.status_code,
            HeaderMap::join_using("\r\n", self.headers.get_map(), additional_headers.get_map()),
            self.body);
    }  

    // Public method
    fn send(&self, mut stream: TcpStream) -> Result<(), AppError> {
        match stream.write_all(self.join().as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(failed!("Error while sending the request")),
        }
    }
}


/// Read the stream and parse it as a request.
///
fn get_request(stream: &TcpStream) -> Result<Request, AppError>  {
    let reader = BufReader::new(stream);    

    // TODO This actually reads only the headers, not the body!
    let raw_head : Result<Vec<String>, _> = reader.lines()
        .take_while(|line| line.is_ok() && !line.as_ref().unwrap().is_empty())
        .collect();

    return if raw_head.is_err() {
        Err(failed!("Could not parse raw request / header lines"))
    } else {
        to_request(raw_head.unwrap().as_mut())
    };
}

fn parse_header_lines(header_lines : &Vec<String>) -> Result<HeaderMap, AppError> {

    let map  = header_lines
        .iter()
        .map(|s| HeaderName::parse(s))
        .collect::<Result<HashMap<HeaderName, String>, AppError>>()
        .map(HeaderMap::from_map)?;

    Ok(map)
}

/// Transform raw header data to a typed request
fn to_request(raw_headers: &mut Vec<String>) -> Result<Request, AppError> {
    if raw_headers.len() < 1 {
        return Err(failed!("Empty request head"));
    } else {
        return identify(&raw_headers[0])
            .map(|(_method, _uri)|
                   Request {
                       method: _method,
                       uri: _uri,
                       headers: parse_header_lines(&raw_headers.split_off(1)).unwrap()
                   });
    }
}

/// Find out the request type by inspecting the start line of an HTTP request
fn identify(start_line: &str) -> Result<(HTTPMethod, String), AppError> {
    // TODO Also scan URL parameters (?foo=x&bar=z)
    //
    // There are other possible values for the start line
    // (https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages#http_requests)
    // We focus on the "Origin Form"
    let re = Regex::new(r"(?<method>[A-Z]+) (?<uri>/\S*) (?<protocol>\S+)").unwrap();
    match re.captures(start_line) {
        None => Err(failed!("Start line did not match the HTTP request origin form")),
        Some(groups) => {
            
            if groups.name("method").is_none()
                || groups.name("uri").is_none()
                || groups.name("protocol").is_none()  {
                    return Err(failed!("In start line, one of 'method', 'uri' or 'protcol' is missing"));
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
                info!("{:?}", get_request(&stream));

                let file = fs::read_to_string("hello.html").unwrap_or("<p>FEHLER BEIM EINLESEN DER DATEI!</p>".to_string());
                
                let response = Response {
                    protocol: "HTTP/1.1".to_string(),
                    status_code: status::StatusCode::OK,
                    headers: HeaderMap::new(),
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
