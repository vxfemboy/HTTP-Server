use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)]
pub enum StatCod {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatCod {
    pub fn resphr(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatCod {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}