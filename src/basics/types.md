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
2's compliment representation

signed range: `-(2n - 1) to 2n - 1`

unsigned range: `0 to 2n - 1`

| Length | Signed | Unsigned |
| ------ | ------ | -------- |
| 8bit    | i8   | u8   |
| 16-bit  | i16  | u16  |
| 32-bit  | i32  | u32  |
| 64-bit  |	i64	 | u64  |
| 128-bit |	i128 | u128 |
| arch    | isize| usize|

## Floating points

IEEE-754 single precision and double precision (f32 and f64)

## Char

Single quotes

4 Bytes - unicode scalar value

U+0000 to U+D7FF and U+E000 to U+10FFFF

## Literals representations

| Number literals |   Example   |
|:---------------:|:-----------:|
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |

## Overflow logic
```rust
let x: u8 = 220;
let (x, ok) = x.overflowing_add(50);
if ok {
		panic!("Overflowing!")
}
println!("{}", x)
```

## Tuples

### Print tuple items
```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
println!("{:?}", tup);
// Pretty print
println!("{:#?}", tup);
```

### Destructure tuple
```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
println!("The value of y is: {}", y);
```

### Access item in tuple
```rust
let x = (10, 10.5);
let y = x.1;
println!("{}", y);
```

## Arrays
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
// Set 5 ints of 5
let b = [5; 5];
println!("{:?}\n{:?}", a, b);
```

## Strings
```rust
let mut s = String::from("hello");
```
`String::from` requests the memory it needs

Deep copy to s2 (copy stack and heap data)
```rust
let s2 = s1.clone();
```

Create an array of bytes from the string
```rust
let bytes = s.as_bytes();
```