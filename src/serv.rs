use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct Serv {
    addr: String,
}

impl Serv {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
    
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Request: {}", String::from_utf8_lossy(&buffer));
                        
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Parse request Failed: {}", e),
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