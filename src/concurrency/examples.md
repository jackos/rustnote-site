
Use threads to print to stdout in parallel
```rust
use std::io::{stdout, Write};
use std::time::Duration;

use rand::Rng;

fn do_work(name: String) {
    let mut rng = rand::thread_rng();
    for _ in 0..40 {
        std::thread::sleep(Duration::from_millis(rng.gen_range(0..=30)));
        print!("{}", name);
        stdout().flush().ok();
    }
}

let a = std::thread::spawn(|| do_work("a".into()));
let b = std::thread::spawn(|| do_work("b".into()));
a.join().unwrap();
b.join().unwrap();
println!();
```
```output
babaabababbabbbaabaaabbbabaaabababaababaababbbaababaaababbabbababaabaababbaabbbb
```
