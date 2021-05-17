use super::serv::Handler;
use super::http::{Request, Response, StatCod, Methd};
use std::fs;

pub struct WebHandler {
    pubpath: String
}

impl WebHandler {
    pub fn new(pubpath: String) -> Self {
        Self { pubpath }
    }

    fn readfile(&self, filepath: &str) -> Option<String> {
        let path = format!("{}/{}", self.pubpath, filepath);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.pubpath) {
                    fs::read_to_string(path).ok()
                }else{
                    println!("Dir Traversal Attack Attempt: {}", filepath);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebHandler {
    fn handreq(&mut self, request: &Request) -> Response {
        match request.methd() {
            Methd::GET => match request.path() {
                "/" => Response::new(StatCod::Ok, self.readfile("index.html")),
                "/hello" => Response::new(StatCod::Ok, self.readfile("hello.html")),
                path => match self.readfile(path) {
                    Some(contents) => Response::new(StatCod::Ok, Some(contents)),
                    None => Response::new(StatCod::NotFound, None),
                },
            },
            _ => Response::new(StatCod::NotFound, None),
        }
    }
}