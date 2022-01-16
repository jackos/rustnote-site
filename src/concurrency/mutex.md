# Arc
Simple example of creating shared reference via Arc to access the same `Vec<u32>` across multiple threads.
```rust
use std::sync::Arc;
use std::thread::{self};

fn main() {
    let numbers: Vec<u32> = (0..100).collect();
    let shared_numbers = Arc::new(numbers); // fix
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = shared_numbers.clone();
        joinhandles.push(thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            while i < child_numbers.len() {
                sum += child_numbers[i];
                i += 8;
            }
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
```


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

## Another example with sleep instead of handles
```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            status_shared.lock().unwrap().jobs_completed += 1;
        }
    });
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
```
