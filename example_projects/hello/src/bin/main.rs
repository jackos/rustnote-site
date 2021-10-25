use hello::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4).expect("Failed to create thread pool");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Made it here");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    // Stream is mutable because it updates internal state after data has been read from it
    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        println!("{}", String::from_utf8_lossy(&buffer));

        let get = b"GET / ";
        let sleep = b"GET /sleep";

        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "../static/hello.html")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "../static/hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "../static/404.html")
        };
        let contents = fs::read_to_string(filename).unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
