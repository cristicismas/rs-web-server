use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn read_and_print_connection(stream: &mut TcpStream, buffer: &mut [u8; 1024]) {
    stream.read(buffer).unwrap();
    
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn respond_with_html(stream: &mut TcpStream) {
    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    read_and_print_connection(&mut stream, &mut buffer);

    respond_with_html(&mut stream);
}
