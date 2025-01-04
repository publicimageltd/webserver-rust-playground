///! Define a header type and some standard headers
///!

use std::{collections::HashMap, fmt};

use crate::AppError;

// Public API

trait HeaderName {
    fn kind(&self) -> Kind;
}

impl HeaderName for String {
    fn kind(&self) -> Kind {
        Kind::Custom(self.to_string())
    }
}
impl HeaderName for PredefinedName {
    fn kind(&self) -> Kind {
        Kind::Standard(*self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Header {
    kind: Kind,
}

impl Header {
    pub fn from<T: HeaderName>(header: T) -> Self {
        Header { kind: header.kind(), }
    }
    /// Parse the given header line into a Header object
    pub fn parse(header_line: &str) -> Result<Header, AppError> {
        if header_line.is_empty() {
            Err(new_err)
        }
    }
}
impl fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug)]
pub struct HeaderMap(HashMap<Header, String>);

impl HeaderMap {
    /// Add a header. A `HeaderName` can be either a string or a
    /// `header::PredefinedName`. 
    pub fn insert<T: HeaderName,V: ToString>(&mut self, header: T, value: V) {
        self.0.insert(Header::from(header), value.to_string());
    }
    /// Join the k v pairs, separated by space, with a seperator
    pub fn join(&self, sep: &str) -> String {
        Self::_join(sep, self.0.iter())
    }
    /// Join two maps using a separator
    pub fn join_using(sep: &str, m1: &HashMap<Header,String>, m2: &HashMap<Header,String>) -> String {
        let iter = m1.iter().chain(m2);
        Self::_join(sep, iter.into_iter())
    }

    // TODO This can be generalized
    /// Join a header hashmap using the iterator passed
    fn _join<'a>(sep: &str, iter: impl Iterator<Item = (&'a Header, &'a String)>) -> String {
        let mut res = String::new();
        let mut peekable = iter.peekable();
        while let Some((k,v)) = peekable.next() {
            res.push_str(&format!("{k} {v}"));
            if peekable.peek().is_some() {
                res.push_str(sep);                
            }
        }
        res
    }

    pub fn new() -> HeaderMap {
        HeaderMap(HashMap::new())
    }
    pub fn get_map(&self) -> &HashMap<Header,String> {
       &self.0
    }
}

// Impl
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Kind {
    Standard(PredefinedName),
    Custom(String),
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Standard(predefined) => write!(f, "{}", predefined),
            Kind::Custom(s) => write!(f, "{}", s),
        }
    }
}

macro_rules! define_standardheaders {
    ( $( ($name:ident, $val:literal), )+   
    ) =>
    { #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum PredefinedName {            
           $( $name, )+
        }
        
      impl fmt::Display for PredefinedName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          match *self {
             $( PredefinedName::$name  => write!(f, $val), )+
          }
        }
      }
    }            
}


define_standardheaders! {
    (ContentLength, "content-length"),
    (Referer, "referer"),    
}


// -----------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn header_macro_as_string() {
        assert_eq!(format!("{}", PredefinedName::ContentLength),
            "content-length")
    }
}
