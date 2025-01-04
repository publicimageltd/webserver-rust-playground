///! Define a header type and some standard headers
///!
///! To add a header, use a HeaderMap and its functions
///! `insert_predefined` and `insert_custom`.
///!

use std::{collections::HashMap, fmt};

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
}
impl fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

pub struct HeaderMap(HashMap<Header, String>);

impl HeaderMap {
    pub fn insert<T: HeaderName,V: ToString>(&mut self, header: T, value: V) {
        self.0.insert(Header::from(header), value.to_string());
    }
    pub fn insert_custom<T: ToString>(&mut self, name: String, value: String) {
        self.0.insert( Header::from_custom(name), value.to_string());
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
