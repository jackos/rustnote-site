# Mutex and Arc
Example of using a `Mutex<T>` with a string, it's responsible for providing mutability and locking the value when already in use. `Arc<T>` is responsible for reference counting when a reference is shared across threads. 
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(String::new()));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += &format!("Thread {} ", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```