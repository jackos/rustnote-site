# Threads

Most basic usage with a `handle.join()`
```rust
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

