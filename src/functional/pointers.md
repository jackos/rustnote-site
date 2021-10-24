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
Function pointers implement all three closure traits: `fn`, `FnMut` and `FnOnce`, so it's best to write a generic parameter using one of those so you can use closures or functions.

## Using function pointer in place of closure
### Closure
```rust
let list_of_strings: Vec<String> =
    list_of_numbers.iter().map(|i| i.to_string()).collect();
```
### Function Pointer
```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> =
	list_of_numbers.iter().map(ToString::to_string).collect();
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

## Return a closure
Must return a pointer with a dynamically sized `Fn` trait
```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```