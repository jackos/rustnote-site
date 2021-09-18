# Types

## Conversions
### Convert string to int
```rust
let mut x = String::new();
stdin().read_line(&mut x).expect("Failed to read line");
let x: u32 = x.trim().parse().expect("Please type a number!");
``` 

### Convert int to string
```rust
let x: i32 = rand::thread_rng().gen_range(1..101);
let x = x.to_string();
```

## Definitions
- Scalar: Single value 
	- integers
	- floating-point
	- booleans
	- characters





## Integer Types
| Length | Signed | Unsigned |
| ------ | ------ | -------- |
| 8bit    | i8   | u8   |
| 16-bit  | i16  | u16  |
| 32-bit  | i32  | u32  |
| 64-bit  |	i64	 | u64  |
| 128-bit |	i128 | u128 |
| arch    | isize| usize|

