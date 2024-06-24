use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
fn main() {
    // binding a listener
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // checking to see if incoming stream from listener has errors
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("message coming");
        handle_connection(stream);
        println!("message done");
    }
    fn handle_connection(mut stream: TcpStream) {
        // we are making a buffer filled with 0s that is of size 1024
        let mut buffer = [0; 1024];
        // thsi puts everything into the buffer? reading the stream
        stream.read(&mut buffer).unwrap();
        // this prints out the whole buffer?
        println!("Request {}", String::from_utf8_lossy(&buffer[..]))
    }
}
