## Error Handling
Put the Result value of read_line into a variable, then match on the enum and run logic depedning on which one it is.
```rust
let stdin_result = std::io::stdin().read_line(&mut guess);
match stdin_result {
    Ok(..) => println!("Cool one!"),
    Err(..) => println!("Oh no!"),
}
```