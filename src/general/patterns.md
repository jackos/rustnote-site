# Patterns
### Simple
A pattern in its most simple form:
```rust
let x = 5;
```
let PATTERN = EXPRESSION;

### Destructure Pattern
```rust
let (x, y, z) = (1, 2, 3);
```
### Ignore Pattern
```rust
let (x, ..) = (1, 2, 3);
// OR
let (x, _, _) = (1, 2, 3);
```

### Destructure Struct
```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```
You could change the variable names like this:
```rust
let Point { x: a, y: b } = p;

println("{} {}", a, b);
```

### Function pattern
Here `&(x, y): &(i32, i32)` is the pattern
```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

### Match values
With an or statement
```rust
let x = 1;

match x {
	1 | 2 => println!("one or two"),
	3 => println!("three"),
	_ => println!("anything"),
}
```
With a range
```rust
let x = 5;

match x {
	1..=5 => println!("one through five"),
	_ => println!("something else"),
}
```

### Match with a destructured struct
```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn where_am_i(&self) {
        match self {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }
}

fn main() {
    let p = Point { x: 8, y: 0 };
    p.where_am_i();

    let p = Point { x: 0, y: 7 };
    p.where_am_i();

    let p = Point { x: 8, y: 7 };
    p.where_am_i();
}
```
### Destructure a struct inside an enum
```rust
enum Message {
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print_me(&self) {
        match self {
            Message::Move { x, y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
        }
    }
}

fn main() {
    let msg = Message::Move { x: 10, y: 15 };
    msg.print_me();

    let msg = Message::ChangeColor(0, 160, 255);
    msg.print_me();
}
```

### Nested enum pattern matching
```rust
use Color::Hsv;
use Color::Rgb;
use Message::ChangeColor;
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    ChangeColor(Color),
}

impl Message {
    fn print_me(&self) {
        match self {
            ChangeColor(Rgb(r, g, b)) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
            ChangeColor(Hsv(r, g, b)) => {
                println!( "Change the color to hue {}, saturation {}, and value {}", r, g, b)
            }
        }
    }
}

fn main() {
    let msg = ChangeColor(Rgb(25, 50, 50));
    msg.print_me();
    let msg = ChangeColor(Hsv(50, 45, 180));
    msg.print_me();
}
```
## Ignoring
### Underscore
Use the underscore to ignore certain patterns
```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    }
}
```

### ..
Use just the x value from a Point:
```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
```

### Match guards
```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

### Complex destructure
Making all values available through a destructure:
```rust
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

