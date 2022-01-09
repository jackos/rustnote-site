## Error Handling
Put the Result value of read_line into a variable, then match on the enum and run logic depedning on which one it is.
```rust
let stdin_result = std::io::stdin().read_line(&mut guess);
match stdin_result {
    Ok(..) => println!("Cool one!"),
    Err(..) => println!("Oh no!"),
}
```
## Shortcut methods
- `unwrap` return value or call panic if error
- `expect` return value or panic with custom message if error 

## Match on error kind / type
```rust
let f = File::open("hello.txt");
let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        other_error => {
            panic!("Problem opening the file: {:?}", other_error)
        }
    },
};
println!("{:?}", f)
```
## More concise nested error with closure
```rust
let f = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
        File::create("hello.txt")
            .unwrap_or_else(|error| panic!("Problem creating the file: {:?}", error))
    } else {
        panic!("Problem opening the file: {:?}", error)
    }
});
```

## Propogate Errors
```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("helo.txt");
    // If error returns error back to caller of method
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // Because last call it returns <String, io::Error>
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    let f = read_username_from_file().expect("Panic with error from main");
    println!("{:?}", f);
}
```
## ? to propogate errors
This is the same as the previous example
? = Ok if all good and continue, return error to calling code if not
```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
Even cleaner!
```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```
Rust has a function already defined for this
```rust
fs::read_to_string("hello.txt");
```

### NOTE
? uses the from function (trait) to convert itself to a standard error type.

### Return error from main
```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    println!("{:?}", f);
    Ok(())
}
```

### Unwrap or else with closure
```rust
let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
```

### Group errors together
So you can return a single error type and then check what specific error it was at the caller via `match` or `if let`
```rust
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    fn from_parse_int(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}
```