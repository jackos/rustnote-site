# Channels
Sharing data to the main thread from a spawned thread
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        tx.send(String::from("cool")).unwrap();
    });

    loop {
        match rx.recv() {
            Ok(message) => println!("Message from spawned thread: {}", message),
            _ => break,
        }
    }
}
```