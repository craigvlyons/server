use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,    
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
        Self::OK => "Ok",
        Self::BadRequest => "Bad request",
        Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }    
}