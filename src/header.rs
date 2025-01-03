///! Headers
//use std::collections::HashMap;

use std::collections::HashMap;

/// (Some) Header names according to the HTTP standard
// We use a lifetime parameter to allow for a constant with global
// (static) lifetime
#[derive(Debug,PartialEq,Hash,Eq)]
pub struct HeaderName<'a>(&'a str);

impl <'a> From<&'a str> for HeaderName<'a> {
    fn from(value: &'a str) -> Self {
        HeaderName(value)
    }
}

pub const CONTENT_LENGTH: HeaderName = HeaderName("content-length");
pub const CONTENT_TYPE:   HeaderName = HeaderName("content-type");
pub const DATE:           HeaderName = HeaderName("date");
pub const FROM:           HeaderName = HeaderName("from");
pub const REFERER:        HeaderName = HeaderName("referer");

struct Headers<'a>(HashMap<HeaderName<'a>,String>);

impl<'a> Headers<'a> {
    fn add_standard(&mut self, name: &'static HeaderName, val: String) {
        HashMap::insert(&mut self.0, HeaderName(name.0), val);
    }
    fn add_custom(&mut self, name: &'a str, val: String) {
        self.0.insert(HeaderName(name), val);
    }
    fn get(self, name: &'a str) -> Option<&'a String> {
        // WTF??? This lifeteime stuff is hell!
        (self.0).get(&HeaderName(name))
    }
}

// -----------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        ()
    }
}


