use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream };
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    let _listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in _listener.incoming() {
        let _stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(_stream); 
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap(); 
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

/*
    HTTP REQUEST FORMAT-
        Method Request-URI HTTP-Version CRLF
        headers CRLF
        message-body
    HTTP RESPONSE FORMAT-
        HTTP-Version Status-Code Reason-Phrase CRLF
        headers CRLF
        message-body
*/

/*
    Turning single-threaded server to multi-threaded server
        - thread Pool
        - fork/join model
        - async I/O model
*/