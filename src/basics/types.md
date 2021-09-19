# Types

## Conversions
### Convert string to int
```rust
let mut x = String::new();
stdin().read_line(&mut x).expect("Failed to read line");
let x: u32 = x.trim().parse().expect("Please type a number!");
``` 

### Convert int to string
```rust
let x: i32 = rand::thread_rng().gen_range(1..101);
let x = x.to_string();
```

## Definitions
- Scalar: Single value 
	- integers
	- floating-point
	- booleans
	- characters


## Integer Types
2's compliment representation

signed range: `-(2n - 1) to 2n - 1`

unsigned range: `0 to 2n - 1`

| Length | Signed | Unsigned |
| ------ | ------ | -------- |
| 8bit    | i8   | u8   |
| 16-bit  | i16  | u16  |
| 32-bit  | i32  | u32  |
| 64-bit  |	i64	 | u64  |
| 128-bit |	i128 | u128 |
| arch    | isize| usize|

## Floating points

IEEE-754 single precision and double precision (f32 and f64)

## Char

Single quotes

4 Bytes - unicode scalar value

U+0000 to U+D7FF and U+E000 to U+10FFFF

## Literals representations

| Number literals |   Example   |
|:---------------:|:-----------:|
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |

## Overflow logic
```rust
let x: u8 = 220;
let (x, ok) = x.overflowing_add(50);
if ok {
		panic!("Overflowing!")
}
println!("{}", x)
```

## Tuples

### Print tuple items
```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
println!("{:?}", tup);
// Pretty print
println!("{:#?}", tup);
```

### Destructure tuple
```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
println!("The value of y is: {}", y);
```

### Access item in tuple
```rust
let x = (10, 10.5);
let y = x.1;
println!("{}", y);
```

## Arrays
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
// Set 5 ints of 5
let b = [5; 5];
println!("{:?}\n{:?}", a, b);
```

## Strings
```rust
let mut s = String::from("hello");
```
`String::from` requests the memory it needs

Deep copy to s2 (copy stack and heap data)
```rust
let s2 = s1.clone();
```

Create an array of bytes from the string
```rust
let bytes = s.as_bytes();
```

## String Slice
String literals are exmaples of string slices `&str`, it is a pointer to a place in memory with a length. The literals point to a place in the binary.

A `String` can be dereference coerced to `&str` as it just takes a slice of the entire String.

A `String` in memory looks like this:

|name|value|
|----|-----|
|ptr|address|
|len|11|
|capacity|11|

Whereas a string slice `&str` looks like this:

|Name | value|
|-----|------|
|ptr|address|
|len|5|

## General Slice
General slices work the same way, it's just a pointer with a length
```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];

println!("{:?}", slice)
```

## Struct
### Define a struct type
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
### Initialize with shorthand
```rust
fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
```
### Use update syntax
```rust
let user1 = build_user(String::from("jack@gmail.com"), String::from("jack"));
let user2 = User {
		email: String::from("newemail@gmail.com"),
		..user1
};
println!("{:#?}\n", user2);
```

### Tuple struct
```rust
struct Point(i32, i32, i32);
let origin = Point(1, 25, 50);
```

### Method on a struct
Mutate itself without taking ownership of itself
```rust
impl Rectangle {
    fn area(&mut self) {
        self.area = self.width * self.height;
    }
}
```
Mutate itself after taking ownership and then return itself

This is rare, could use it if transforming self into something very different, which would force the user to initialize it to another variable.
```rust
impl Rectangle {
    fn area(mut self) -> Rectangle {
        self.area = self.width * self.height;
        self
    }
}
```

### Associated Functions
Function on a struct that doesn't use self, generallly return an instance of themselves e.g.
```rust
impl Rectangle { 
	fn square(size: u32) -> Rectangle {
		Rectangle {
			width: size,
			height: size,
			area: 0,
		}
	}
}

let square = Rectangle::square(5);
```

## Enums

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

### Enum methods
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
        // method body would be defined here
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```