# Modules

## Absolute and relative paths

- `crate::` = root of current crate
- `super::` = from parent module, where root of crate is also considered a module

```rust
pub mod front_of_house {
		// Doesn't have to be public because it's a sibling
    mod hosting {
				// Needs to be public as it's a child we want to call from outer scope
        pub fn add_to_waitlist() {}
    }

    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();
				// Relative path
        hosting::add_to_waitlist()
    }
}
```
## Structs
Struct fields are private by default, you can't read or write from a parent. the summer() function constructs and returns a breakfast struct, but the private field still can't be used directly from any parents.
```rust
pub mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {:?} please", meal);
}
```

## Enums
Enums are public by default
```rust
mod back_of_house {
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("order1: {:?}\norder2: {:?}", order1, order2);
}
```

## Use
Idiomatic to only bring in the parents of functions, so it's clear where they come from from
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to waitlist")
        }
    }
}

// Not idiomatic
use front_of_house::hosting::add_to_waitlist;
pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

// Idiomatic
use front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

Structs, enums, and other types it's OK to bring in the full path
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```
## As
If two things have the same name, the 'as' keyword can be used.
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

## pub use
This will export the used path, so in the example below hosting will be available
```rust
// lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

// main.rs
use restaurant::hosting;
fn main() {
    hosting::add_to_waitlist()
}
```

## Nested paths
```rust
use std::{collections::HashMap, vec};

fn main() {
    let mut map = HashMap::new();
    let mut vec = vec![1, 2, 3, 4];
    vec.push(10);
    map.insert(1, 2);

    println!("{:?}\n{:?}", map, vec);
}
```
## Over the top example of nested paths
```rust
use std::{
    fs::File,
    io::{
        self,
        prelude::{Read, Seek},
        SeekFrom,
    },
};

fn main() -> io::Result<()> {
    let mut f = File::open("./src/test.txt")?;
    let mut buffer = [0; 10];
    f.seek(SeekFrom::End(-10))?;
    let n = f.read(&mut buffer)?;
    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}
```
## Glob pattern
\* brings in everything without having to suffix it

The above could be written as
```rust
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
```
Which is much nicer to look at and understand

## Include another file
Putting mod in front of a filename
```rust
// src/front_of_house.rs OR
// src/front_of_house/mod.rs
pub fn add_to_waitlist() {
    println!("Wow cool yo")
}

// src/lib.rs
// mod will look for file front_of_house.rs or front_of_house/mod.rs
mod front_of_house;
pub use front_of_house::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}