///! Headers
//use std::collections::HashMap;

use std::collections::HashMap;

/// (Some) Header names according to the HTTP standard
// We use a lifetime parameter to allow for a constant with global
// (static) lifetime
#[derive(Debug,PartialEq)]
pub struct HeaderName<'a>(&'a str);

pub const CONTENT_LENGTH: &HeaderName = &HeaderName("content-length");
pub const CONTENT_TYPE:   &HeaderName = &HeaderName("content-type");
pub const DATE:           &HeaderName = &HeaderName("date");
pub const FROM:           &HeaderName = &HeaderName("from");
pub const REFERER:        &HeaderName = &HeaderName("referer");

/// A header, both for responses or requests.
///
/// Note that HTTP 2 allows non-string headers.

#[derive(Debug,PartialEq)]
struct Header<'a> {
    name: HeaderName<'a>,
    // TODO Non-empty string, actually.
    value: &'a str,
}

impl <'a> Header<'a> {
    fn custom(name: &'a str, val: &'a str) -> Header<'a> {
        Header { name: HeaderName(name),
            value: val,
        }
    }
    fn standard(name: &'static HeaderName, val: &'a str) -> Header<'a> {
        Header { name: HeaderName(name.0),
            value: val }
    }
}


// -----------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_custom_headers() {
        let custom = Header::custom("date", "today");
        let standard = Header::standard(DATE, "today");
        assert_eq!(custom, standard);
    }
}


