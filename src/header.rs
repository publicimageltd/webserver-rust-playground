///! Headers
use std::fmt;

/// (Some) Header names according to the HTTP standard
macro_rules! define_standardheaders {
    ( $( ($name:ident, $val:literal), )+   
    ) =>
    { #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
      enum StandardHeader {
      $( $name, )+
      }
        
      impl fmt::Display for StandardHeader {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          match *self {
             $( StandardHeader::$name  => write!(f, $val), )+
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
        assert_eq!(format!("{}", StandardHeader::ContentLength),
            "content-length")
    }
}
