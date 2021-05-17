use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{ParseErr, Request, Response, StatCod};
use std::convert::TryFrom;

pub trait Handler {
    fn handreq(&mut self, req: &Request) -> Response;

    fn handbadreq(&mut self, e: &ParseErr) -> Response {
        println!("Failed to Parse Req: {}", e);
        Response::new(StatCod::BadRequest, None)
    }
}

pub struct Serv {
    addr: String,
}

impl Serv {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
    
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Request: {}", String::from_utf8_lossy(&buffer));
                        
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handreq(&request),
                                Err(e) => handler.handbadreq(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                               println!("Failed to send resp: {}", e);
                           }
                        }
                        Err(e) => println!("Failed to read connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to make connection: {}!", e),
            }
            
        }
    }
}