use std::fs;
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
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        // specify the url path after the slash
        let get = b"GET / HTTP/1.1\r\n";
        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
        let contents = fs::read_to_string(filename).unwrap();
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",status_line, contents.len(), contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
