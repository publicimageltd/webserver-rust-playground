///! Define a header type and some standard headers
///!
///! For some obscure reasons, a "Header" is not just a tuple of
///! Strings, as one would expect. The author of this file has
///! decided to distinguish between predefined standard headers,
///! and custom headers with an unknown name. Thus, a header is:
///!
///! ```
///! let header = (HeaderName, String);
///! ```
///!
///! `HeaderName` is actually a trait which is implemented for
///! the String type and the enum with standard headers,
///! `PredefinedName`. There are some convenience functions to
///! create and inspect `HeaderName`s.
///! 
///! Further, to collect a list of headers, this module provides a
///! `HeaderMap`.
///! 
///! This whole thing is not really elegant nor useful. The original
///! idea was to somehow parse known headers, but in retrospect,
///! this is not useful. A better approach would be to
///! collect all header lines in one string and then actively search
///! for a certain header name via regex. After all, the server is
///! only /responding/ to certain headers, so there is no need to
///! preemptively encode all headers. So we would need no map,
///! and no special types, just a string buffer. (Unless we switch to
///! HTTP 2, that is. But that's another story.)

use std::{collections::HashMap, fmt};

use crate::info;
use crate::AppError;
use crate::failed;

pub trait IntoHeaderName {
    fn kind(&self) -> Kind;
}

impl IntoHeaderName for String {
    fn kind(&self) -> Kind {
        Kind::Custom(self.to_string())
    }
}
impl IntoHeaderName for PredefinedName {
    fn kind(&self) -> Kind {
        Kind::Standard(*self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeaderName {
    pub kind: Kind,
}

impl HeaderName {
    
    pub fn is_custom(&self) -> bool {
        match self.kind {
            Kind::Standard(_) => false,
            Kind::Custom(_) => true,
        }
    }

    pub fn is_standard(&self) -> bool {
        !self.is_custom()
    }
    
    fn from<T: IntoHeaderName>(header: T) -> Self {
        HeaderName { kind: header.kind(), }
    }
    /// Parse the given header line into a header tuple 
    pub fn parse(header_line: &str) -> Result<(HeaderName, String), AppError> {
        match header_line.split_once(":") {
            Some((name, value)) => {
                match PredefinedName::find(name) {
                    Some(predefined_name) => Ok((predefined_name, value.trim().to_string())),
                    None => Ok((HeaderName::from(name.to_string()), value.trim().to_string()))
                }
            }
            None => Err(failed!("No  colon found; could not split"))
        }        
    }
}
impl fmt::Display for HeaderName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug)]
pub struct HeaderMap(HashMap<HeaderName, String>);

impl HeaderMap {
    /// Add a header. A `HeaderName` can be either a string or a
    /// `header::PredefinedName`. 
    pub fn insert<T: IntoHeaderName,V: ToString>(&mut self, header: T, value: V) {
        self.0.insert(HeaderName::from(header), value.to_string());
    }
    /// Join the k v pairs, separated by space, with a seperator
    pub fn join(&self, sep: &str) -> String {
        Self::_join(sep, self.0.iter())
    }
    /// Join two maps using a separator
    pub fn join_using(sep: &str, m1: &HashMap<HeaderName,String>, m2: &HashMap<HeaderName,String>) -> String {
        let iter = m1.iter().chain(m2);
        Self::_join(sep, iter.into_iter())
    }

    // TODO This can be generalized
    /// Join a header hashmap using the iterator passed
    fn _join<'a>(sep: &str, iter: impl Iterator<Item = (&'a HeaderName, &'a String)>) -> String {
        let mut res = String::new();
        let mut peekable = iter.peekable();
        while let Some((k,v)) = peekable.next() {
            let new_val = format!("{k}: {v}");
            res.push_str(&new_val);
            if peekable.peek().is_some() {
                res.push_str(&sep);                
            }
        }
        res
    }

    pub fn new() -> HeaderMap {
        HeaderMap(HashMap::new())
    }
    pub fn get_map(&self) -> &HashMap<HeaderName,String> {
       &self.0
    }
    pub fn from_map(map: HashMap<HeaderName,String>) -> Self {
        HeaderMap(map)
    }
}

// Impl
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Kind {
    Standard(PredefinedName),
    Custom(String),
}

impl Kind {
    fn test() {
        ()
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Standard(predefined) => write!(f, "{}", predefined),
            Kind::Custom(s) => write!(f, "{}", s),
        }
    }
}

///! Ease the definition of standard headers.
///!
///! ```
///! define_standardheaders! {
///!     (ContentLength, "content-length"),
///!     (Referer, "referer"),    
///! }
///! ```
///! 
///! For each tuple, the first value becomes a valid value for the
///! enum `PredefinedName`, and the second one is a string
///! representation which is used both for printing and
///! parsing header names.
///! 
///! For parsing a raw text into a HeaderName object and its
///! associated value, use `PredefinedName::find`.
///! 
macro_rules! define_standardheaders {
    ( $( ($name:ident, $val:literal), )+   
    ) =>
    { #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum PredefinedName {            
           $( $name, )+
        }

        impl PredefinedName {
            pub fn find(s: &str) -> Option<HeaderName> {
                let mut needle: String = String::from(s);
                needle.make_ascii_lowercase();
//                println!("Comparing against {}", &needle);
                match needle.as_ref() {
                    $( $val => Some(HeaderName::from(PredefinedName::$name)), )+
                    _ => None
                }
            }
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
    (Server, "server"),
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
