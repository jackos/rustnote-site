# Traits

## Default implementation
NewsArticle implements Summary, but defines it's own version. Tweet uses the default implementation by using an empty block.
```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {}
```
## Functions
### Function implementing interface
Now we can set function that accepts anything which implements Summary
```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
### Long form
The trait bound function is using short form impl keyword, if we wanted to constrain two arguments to use the same type we'd need to use the long form:
```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {}
```
### Constrain to multiple traits:
```rust
// short form
pub fn notify(item: &(impl Summary + Display)) {}
// long form
pub fn notify<T: Summary + Display>(item: &T) {
```
### Where clause
Implementing multiple interfaces across multiple arguments can become long
```rust 
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```
There is alternate syntax to make this clearer
```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}
```
## Return trait
```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

## Default implementation calling
There is no problem for a default implementation to call a function that hasn't been implemented. This allows the user of a trait to only have to define a small amount of logic.
```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

### Conditionally implement methods
You can implement a method, only when the type has implemented a trait, the below is an example of a implementing cmp_display() when a the type being used inside `Pair` has implemented `Display` and `PartialOrd`
```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```
## Advanced

### Placeholder Types (associated type)
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```
This can be used like:
```rust

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
```
Where a u32 is being returned

### Operator Overloading
This example is similar in that there is an Output placeholder type that determines what is returned.
```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```
This is what the `Add` trait looks like:
```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```
Notice the `Add<Rhs=Self>`, this is called a `Default Generic Type Parameter`, below we will override the default of `Self` to add two different types:
```rust
use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    println!("{:?}", Point { x: 1, y: 25 } + Point { x: 5, y: 10 });
    let x = Millimeters(2550);
    let y = Meters(3);
    println!("{:?}", x + y);
}
```

## Fully Qualified Disambiguation
When there are multiple implementations of traits with the same method name, you need to fully qualify so Rust knows which one to use:
```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;
    person.fly();
    Wizard::fly(&person);
    Pilot::fly(&person);
}
```
Note that this only works because `&self` is passed in, if it doesn't have `&self` i.e. there's no object that it's attached to, you have to use a different syntax:
```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```
`<Dog as Animal>::baby_name())` is refereed to as fully qualified syntax

The spec is:
```rust
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

## Supertraits
Below we use `self.to_string()`, that method only appears as part of `fmt::Display`, which is added as a supertrait with `OutlinePrint: fmt::Display`. Now before `OutlinePrint` can be implemented on `Point`, we also need to implement `fmt::Display`
```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn main() {
    let x = Point { x: 25, y: 10 };
    x.outline_print();
}
```

## Newtype Pattern with Traits
Use external traits with external types by wrapping it in a new type like so:
```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[\n  {}\n]", self.0.join("\n  "))
    }
}

fn main() {
    let w = Wrapper(vec![
        String::from("hello"),
        String::from("world"),
        String::from("coolio"),
        String::from("coolio"),
    ]);
    println!("w = {}", w);
}
```
This allows us to override `Display` on the `Vec` type, if you then want to get all the methods on `Vec<String>` just override the deref:
```rust
impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```