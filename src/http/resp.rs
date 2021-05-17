use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::TcpStream;
use std::io::{Write, Result as IoRes};
use super::StatCod;

#[derive(Debug)]
pub struct Response {
    statcod: StatCod,
    body: Option<String>,
}

impl Response {
    pub fn new(statcod: StatCod, body: Option<String>) -> Self {
        Response { statcod, body}
    }

    pub fn send(&self, stream: &mut impl Write) -> IoRes<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream, 
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.statcod,
            self.statcod.resphr(),
            body
        )
    }
} 
