use std::fmt;

///! HTTP Response Status Codes

#[derive(Debug,Copy,Clone)]
pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
    URITooLong = 414,
    NotImplemented = 0,
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reason: String = match *self {
            StatusCode::OK => "OK",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
            StatusCode::URITooLong => "URI Too Long",
            _ => "",
        }.to_string();
        write!(f, "{} {}", *self as u16, reason)
    }
}
