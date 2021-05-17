use super::methd::{Methd, MethdErr};
use super::{QStr};
use std::convert::TryFrom;
use std::str;
use std::str::Utf8Error;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QStr<'buf>>,
    methd: Methd,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn methd(&self) -> &Methd {
        &self.methd
    }

    pub fn query_string(&self) -> Option<&QStr> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseErr;

    // GET /test?querystr=char&bleh=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let req = str::from_utf8(buf)?;

        let (methd, req) = gnxtwrd(req).ok_or(ParseErr::InvalidRequest)?;
        let (mut path, req) = gnxtwrd(req).ok_or(ParseErr::InvalidRequest)?;
        let (protocol, _) = gnxtwrd(req).ok_or(ParseErr::InvalidRequest)?;
        
        if protocol != "HTTP/1.1" {
            return Err(ParseErr::InvalidProtocol);
        }
        
        let methd: Methd = methd.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QStr::from(&path[i+1..]));
            path = &path[..1];
        }

        Ok(Self {
            path,
            query_string,
            methd,
        })
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

impl From<MethdErr> for ParseErr {
    fn from(_: MethdErr) -> Self {
        Self::InvalidEncoding
    }
}

impl From<Utf8Error> for ParseErr {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseErr {}