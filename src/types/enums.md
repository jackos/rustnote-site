# Enums

### Standard

```rust
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:?}", home)
}
```

### Associated data

```rust
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}{:?}", home, loopback)
}
```

### Enum method

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("self: {:?}", self);
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```

### Match logic with enum

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn main() {
    let x = Coin::Dime;
    println!("{}", x.value_in_cents())
}
```

### Enum inside enum, binds to match branch

```rust
#[derive(Debug)]
enum UsState {
    Alabambda,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(x) => {
                println!("Comes from {:?}", x);
                25
            }
        }
    }
}

fn main() {
    let x = Coin::Quarter(UsState::Alaska);
    println!("{}", x.value_in_cents())
}
```

### Option Some None enum match

This is how you check for a none / null /nil in Rust

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        // _ matches all remaining values
        // _ => () will do nothing
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}\n{:?}\n{:?}", five, six, none);
}
```

### if let matching

`if let` is syntactic sugar for checking a match and running logic if true, then ignore all other values.

```rust
let x = Some(3);

// match on x if == Some(3), else do nothing
match x {
    Some(num) => println!("number: {}", num),
    _ => (),
}

// Exact same logic with if let
if let Some(num) = x {
    println!("number: {}", num)
// Can also throw in an else statement
} else {
    println!("not three");
}

```