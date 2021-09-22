# Vector
## Creating
### Specify the type
```rust
let v: Vec<i32> = Vec::new();
```
### Infer with first type pushed in
```rust
let mut v = Vec::new();
v.push(1);
```
### Infer the type with a macro
```rust
let v = vec![1, 2, 3];
```
## Get elements
### Get element directly
```rust
let third: &i32 = &v[2];
```
### Match option check
Do a check to see if element is there
```rust
match v.get(2) {
	Some(third) => println!("The third element is {}", third),
	None => println!("There is no third element."),
}
```

## Loop over vector
```rust
let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
    }
```

## Enum for different types
```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
```
Match on the different types
```rust
 for x in row {
    match x {
        SpreadsheetCell::Int(x) => println!("{}", x),
        SpreadsheetCell::Text(x) => println!("{}", x),
        SpreadsheetCell::Float(x) => println!("{}", x),
    }
}
```