## Conversions

### String to int

```rust
let mut x = String::new();
stdin().read_line(&mut x).expect("Failed to read line");
let x: u32 = x.trim().parse().expect("Please type a number!");
```

### int to String

```rust
let x: i32 = rand::thread_rng().gen_range(1..101);
let x = x.to_string();
```

```rust
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
```

### Vectors to HashMap
```rust
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
```