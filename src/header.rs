///! Define a header type and some standard headers
///!
///! To get a header, use `header::predefined()` or `header::custom`
///!

use std::fmt;

// Public API
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Header {
    kind: Kind,
}

pub fn predefined(header: Predefined) -> Header {
    Header { kind: Kind::Standard(header), }
}

pub fn custom(s: String) -> Header {
    Header { kind: Kind::Custom(s), }
}

// Impl
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Kind {
    Standard(Predefined),
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
        pub enum Predefined {            
           $( $name, )+
        }
        
      impl fmt::Display for Predefined {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          match *self {
             $( Predefined::$name  => write!(f, $val), )+
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
        assert_eq!(format!("{}", Predefined::ContentLength),
            "content-length")
    }
}
