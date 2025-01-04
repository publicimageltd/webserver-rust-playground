///! Define a header type and some standard headers
///!
///! To add a header, use a HeaderMap and its functions
///! `insert_predefined` and `insert_custom`.
///!

use std::{collections::HashMap, fmt};

// Public API

pub trait HeaderName {
    fn to_string(&self) -> String;
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Header {
    kind: Kind,
}

impl Header {
    fn from_predefined(header: PredefinedName) -> Self {
        Header { kind: Kind::Standard(header), }
    }
    fn from_custom(name: String) -> Self {
        Header { kind: Kind::Custom(name), }
    }
}

pub struct HeaderMap(HashMap<Header, String>);

impl HeaderMap {
    pub fn insert_predefined<T: ToString>(&mut self, header: PredefinedName, value: T) {
        self.0.insert( Header::from_predefined(header), value.to_string());
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
    { #[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
