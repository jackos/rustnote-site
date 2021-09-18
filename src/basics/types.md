## Types
### Convert string input to int
```rust
let mut input = String::new();
std::io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

let trimmed = input.trim();
let multiplied = trimmed.parse::<u32>().unwrap() * 10;
println!("Result: {}{}", input, multiplied)
```

### Convert with error checking
```rust
let input = "20";
let trimmed = input.trim();
let mut apples: u32 = 0;
match trimmed.parse::<u32>() {
    Ok(i) => apples = i * 10,
    Err(..) => println!("this was not an integer: {}", trimmed),
};

println!("Result: {} * 10 = {} apples", input, apples)
```