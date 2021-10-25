# main.rs
```rust
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(&stream);
    }
    // Stream is mutable because it updates internal state after data has been read from it
    fn handle_connection(mut stream: &TcpStream) {
        let mut buffer = [0; 1028];
        let x = stream.read(&mut buffer).unwrap();
        println!(
            "\n---------------\nBytes read: {}\n---------------\n{}",
            x,
            String::from_utf8_lossy(&buffer[..])
        );
    }
}
```
### TcpStream
This needs to be mutable because it keeps a track of its own state. When data is read to a buffer, it's removed from the stream. If there's not enough room in the buffer, it remains in the stream.

### Response
If you navigate to "127.0.0.1:7878" it will return this response:
```text
---------------
Bytes read: 558
---------------
GET / HTTP/1.1
Host: 127.0.0.1:7878
Connection: keep-alive
Cache-Control: max-age=0
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.4577.82 Safari/537.36
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9
Sec-GPC: 1
Sec-Fetch-Site: none
Sec-Fetch-Mode: navigate
Sec-Fetch-User: ?1
Sec-Fetch-Dest: document
Accept-Encoding: gzip, deflate, br
Accept-Language: en-US,en;q=0.9
```

HTTP requests are in this format:
```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

