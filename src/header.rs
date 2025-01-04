///! Define a header type and some standard headers
///!

use std::{collections::HashMap, fmt};

use crate::AppError;
use crate::failed;

// Public API

trait IntoHeaderName {
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
    kind: Kind,
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
    
    pub fn from<T: IntoHeaderName>(header: T) -> Self {
        HeaderName { kind: header.kind(), }
    }
    /// Parse the given header line into a header tuple 
    pub fn parse(header_line: &str) -> Result<(HeaderName, String), AppError> {
        match header_line.split_once(":") {
            Some((name, value)) => {
                match PredefinedName::find(name) {
                    Some(predefined_name) => Ok((predefined_name, value.to_string())),
                    None => Ok((HeaderName::from(name.to_string()), value.to_string()))
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
    pub fn get_map(&self) -> &HashMap<HeaderName,String> {
       &self.0
    }
    pub fn from_map(map: HashMap<HeaderName,String>) -> Self {
        HeaderMap(map)
    }
}

// Impl
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Kind {
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

macro_rules! define_standardheaders {
    ( $( ($name:ident, $val:literal), )+   
    ) =>
    { #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum PredefinedName {            
           $( $name, )+
        }

        impl PredefinedName {
            pub fn find(s: &str) -> Option<HeaderName> {
                match s {
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
