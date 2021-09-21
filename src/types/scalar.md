# Scalar Types

Scalar = single values: integers, floating-point, booleans, characters

## Integer Types

2's compliment representation

signed range: `-(2n - 1) to 2n - 1`

unsigned range: `0 to 2n - 1`

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8bit    | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

### Overflow logic

```rust
let x: u8 = 220;
let (x, ok) = x.overflowing_add(50);
if ok {
		panic!("Overflowing!")
}
println!("{}", x)
```

## Floating points

IEEE-754 single precision and double precision (f32 and f64)

## Char

Single quotes

4 Bytes - unicode scalar value

U+0000 to U+D7FF and U+E000 to U+10FFFF

## Literals representations

| Number literals |   Example   |
| :-------------: | :---------: |
|     Decimal     |   98_222    |
|       Hex       |    0xff     |
|      Octal      |    0o77     |
|     Binary      | 0b1111_0000 |
| Byte (u8 only)  |    b'A'     |
