# Lifetimes

## Basic
'a denotes the lifetime, this tells the compiler that whatever comes in will share a lifetime with what goes out.
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
This makes the return value valid for the smaller of the lifetimes of x and y.

## Error
If any of the parameters (which are references) don't live as long as the return value, it will cause an error. Because that could be referencing a dangling pointer. 
```rust
let string1 = String::from("long string is long");
let result;
{
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
}
println!("The longest string is: {}", result);
```

## Structs
```rust
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
    author: &'a String,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let author;
    {
        author = String::from("Jacky C Yo");
    }
    let i = ImportantExcerpt {
        part: first_sentence,
        author: &author,
    };
    println!("{:?}", i);
}
```

### Lifetime elision
Inference the compiler does on some lifetime patterns e.g.
```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```
The compile is able to determine the signature of this is:
```rust
fn first_word<'a>(s: &'a str) -> &'a str{}
```
Because it's simply returning a reference that lives the same amount of time as the one it returns.

Parameters = input lifetimes
Return values = output lifetimes

## Lifetimes Inference rules:

1) Each input parameters gets its own lifetime parameter e.g. `a'`
2) If there is exactly one input lifetime parameter, the lifetime is assigned to all output parameters 
3) If self in method and multiple parameters, all output lifetime parameters become self

### 3rd rule example
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("{}{}", announcement, self.part);
        self.part
    }
}
```
Annotating the function with 'a is not required because it satisfies the lifetime inference rules.

### Static
Tells the compile that the reference lives for the entire duration of the program.

E.g. all `&str` live inside the binary, so they already have the lifetime of `'static`:
```rust
// Redundant, compiler already knows this is 'static
let s: &'static str = "I have a static lifetime.";
```

## Example of generic type, trait and lifetime
```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = longest_with_an_announcement("Wow very cool", "Not cool", "You win!");
    println!("{}", x)
}
```