
# Function Pointers
Some languages refer to this ass first class functions i.e. functions can be passed around.
## Example
```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}
```
```output
The answer is: 12
```

Function pointers implement all three closure traits: `fn`, `FnMut` and `FnOnce`, so it's best to write a generic parameter using one of those so you can use closures or functions.

## Using function pointer in place of closure
### Closure
```rust
let list_of_numbers = vec![12, 10, 4, 54];
let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
println!("{list_of_strings:?}");
```
```output
["12", "10", "4", "54"]
```

### Function Pointer
```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

println!("{list_of_strings:?}");
```
```output
["1", "2", "3"]
```

Standard lib implements `ToString` for any type that implements `Display`

## Passing initializers as functions
When you initialize a struct or an enum, you're actually calling a function that returns an instance that's constructed from the arguments. So you can pass an initialization to something accepting `Fn` or `fn`, for example below with `Status::Value`
```rust
#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let mut list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    list_of_statuses.push(Status::Stop);
    println!("{:?}", list_of_statuses);
}
```
```output
[Value(0), Value(1), Value(2), Value(3), Value(4), Value(5), Value(6), Value(7), Value(8), Value(9), Value(10), Value(11), Value(12), Value(13), Value(14), Value(15), Value(16), Value(17), Value(18), Value(19), Stop]
```

## Return a closure
Must return a pointer with a dynamically sized `Fn` trait
```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

let x = returns_closure();
let y = x(4);

println!("{y}");
```
```output
5
```
