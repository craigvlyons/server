use super::method::Method;
use std::str::Utf8Error;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug};
use std::fmt::Result as fmtResult;
use std::fmt::Formatter;
use std::str;

pub struct Request{
    path: String,
    query_string: Option<String>,
    method: Method,
}


impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // get/search?name=abc&sort=1 http/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;
            
        unimplemented!()
    }
}


pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmtResult{
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {
        
}