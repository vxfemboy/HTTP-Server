#![allow(dead_code)]

use serv::Serv;
use std::env;
use webhandler::WebHandler;

mod serv;
mod http;
mod webhandler;


fn main() {
    let defpath = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let pubpath = env::var("PUBPATH").unwrap_or(defpath);
    println!("pubpath: {}", pubpath);
    let serv = Serv::new("127.0.0.1:1337".to_string());
    serv.run(WebHandler::new(pubpath));
}