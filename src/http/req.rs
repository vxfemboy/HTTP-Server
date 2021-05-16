use super::methd::Method;
use std::convert::TryFrom;
use std::str;
use std::str::Utf8Error;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseErr;

    // GET /test HTTP/1.1\r\n...HEADERS...
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let req = str::from_utf8(buf)?;

        let (methd, req) = gnxtwrd(req).ok_or(ParseErr::InvalidRequest)?;
        let (path, req) = gnxtwrd(req).ok_or(ParseErr::InvalidRequest)?;
        let (protocol, _) = gnxtwrd(req).ok_or(ParseErr::InvalidRequest)?;
        
        if protocol != "HTTP/1.1" {
            return Err(ParseErr::InvalidProtocol);
        }
        unimplemented!()
    }
}

fn gnxtwrd(req: &str) -> Option<(&str, &str)> {
    for (i, c) in req.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&req[..i], &req[i+1..]));
        }
    }
    None
}

pub enum ParseErr {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.msg())
    }
}

impl Debug for ParseErr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.msg())
    }
}

impl ParseErr {
    fn msg(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<Utf8Error> for ParseErr {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseErr {}