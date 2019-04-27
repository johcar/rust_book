use hello::Threadpool;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = Threadpool::new(4);

    for stream in listener.incoming().take(5) {
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    //  create a buffer and read to it
    let mut buffer = [0; 512];
    stream.read(&mut buffer).expect("failed on request read");

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // is this a GET request or some other request?
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).expect("failed on file read");
    let response = format!("{}{}", status_line, contents);

    // write the response back to the client
    stream
        .write(response.as_bytes())
        .expect("failed on write response");
    stream.flush().expect("failed on flush stream");
}
