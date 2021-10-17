# fs

## Open file and write to it
```rust
use std::fs::File;
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("helo.txt");
    let mut f = match f {
        Ok(file) => file,
        // Return early if there's an error
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // Because last call it returns <String, io::Error>
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```
## Read from a file the long way
```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
## Read from a file the short way
```rust
use std::fs::read_to_string;
let x = fs::read_to_string("hello.txt");
println!(x)
```