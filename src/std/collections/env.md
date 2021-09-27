# env

## Get arguments from command
This will panic if the command contains invalid unicode, use `std::env::args_os` instead
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```