///! Headers
use std::collections::HashMap;

/// (Some) Header names according to the HTTP standard
// We use a lifetime parameter to allow for a constant with global
// (static) lifetime
#[derive(Debug)]
pub struct HeaderName<'a>(&'a str);

pub const CONTENT_LENGTH: HeaderName = HeaderName("content-length");
pub const CONTENT_TYPE:   HeaderName = HeaderName("content-type");
pub const DATE:           HeaderName = HeaderName("date");
pub const FROM:           HeaderName = HeaderName("from");
pub const REFERER:        HeaderName = HeaderName("referer");

/// A header, both for responses or requests.
///
/// Note that HTTP 2 allows non-string headers.

#[derive(Debug)]
struct Header<'a> {
    name: HeaderName<'a>,
    // TODO Non-empty string, actually.
    value: &'a str,
}



