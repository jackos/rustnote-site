# Closures


### Function definition compared to closures

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

## Test adding a header here
```rust
let x = vec![1, 5, 654, 78];
for item in x {
	println!("Nice one: {}", item)
}
```
```output
Nice one: 1
Nice one: 5
Nice one: 654
Nice one: 78
```

### Capturing value from environment
This is an example where the value from the surrounding environment is used in the closure.

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| x == z;

    let y = 4;
    println!("result: {}", equal_to_x(y));
}
```
### Closure Traits
These are inferred from closure usage
- `FnOnce` takes ownership of variable from surrounding environment, means the closure can't be called more than once
- `FnMut` borrows mutably
- `Fn` borrows immutably 

### move
Use the `move` keyword to ensure we transfer ownership to the closure, if it's a scalar value it will create a new value

```rust
fn main() {
    let x = 4;

    let equal_to_x = move |z| x == z;

    let y = 4;
    println!("result: {}", equal_to_x(y));
}
```

### Closure use example
Create a cache to hold the result from a closure in a map

This allows to not run the same expensive function more than once for the same value

```rust
use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;
struct Cacher<T, U>
where
    T: Fn(U) -> U,
{
    calculation: T,
    values: HashMap<U, U>,
}

impl<T, U: Eq + Hash + Copy> Cacher<T, U>
where
    T: Fn(U) -> U,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }
    fn values(&mut self, arg: U) -> U {
        if !self.values.contains_key(&arg) {
            self.values.insert(arg, (self.calculation)(arg));
        }
        self.values[&arg]
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let closure = |num| {
        println!("calculating slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let mut expensive_result = Cacher::new(closure);
    // Example only running expensive result closure when needed
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.values("cool"));
        println!("Next, do {} situps!", expensive_result.values("cool"));
        println!("Next, do {} pullups!", expensive_result.values("very cool"));
    } else {
        // Example not having to run expensive calculation at all
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.values("so damn cool")
            )
        }
    }
}
fn main() {
    let intensity = 10;
    let simulated_random_number = 7;
    generate_workout(intensity, simulated_random_number);
}
```
