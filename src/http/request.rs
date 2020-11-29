use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Debug};
use std::fmt::Result as FmtResult;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

// Our custom error type
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl ParseError {
    // Type has has one private method
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method"
        }
    }
}
// implement the error trait (interface) from std library
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {

}