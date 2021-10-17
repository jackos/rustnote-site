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

## Conditionally implement methods
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