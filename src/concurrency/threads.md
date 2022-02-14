
# Threads

Most basic usage with a `handle.join()`
```rust
use std::thread;
use std::time::Duration;
let x = String::new();
let handle = thread::spawn(|| {
	for i in 1..10 {
		println! {"Spawned thread: {}", i}
		thread::sleep(Duration::from_millis(1));
	}
});

for i in 1..5 {
	println!("Main thread: {}", i);
	thread::sleep(Duration::from_millis(1));
}

handle.join().expect("Something went wrong in thread");
println!("This will print after the thread has finished");
```
```output
Main thread: 1
Spawned thread: 1
Main thread: 2
Spawned thread: 2
Main thread: 3
Spawned thread: 3
Main thread: 4
Spawned thread: 4
Spawned thread: 5
Spawned thread: 6
Spawned thread: 7
Spawned thread: 8
Spawned thread: 9
This will print after the thread has finished
```

## Passing ownership to thread
```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```
