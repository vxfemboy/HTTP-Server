use serv::Serv;
use http::Request;
use http::Methd;

mod serv;
mod http;


fn main() {
    let serv = Serv::new("127.0.0.1:1337".to_string());
    serv.run();
}