
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

## Getter / Setter
This is an example of how you could implement OOP style getters and setters with checks.
```rust,editable
#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let guess = Guess::new(10);

    println!("{:?}", guess.value())
}
```