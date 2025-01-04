use std::{error::Error, fmt};

#[derive(Debug)]
pub struct AppError {
    pub line: u32,
    pub file: &'static str,
    pub info: String,
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} {}", self.file, self.line, self.info)
    }
}

impl Error for AppError {}

#[macro_export]
macro_rules! failed {
    ( $($arg:expr),+ ) =>
       {
           crate::error::AppError {
               line: line!(),
               file: file!(),
               info: format!($($arg),+)
           }
       }    
}
