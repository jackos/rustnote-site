# Closures

### Function definition compared to closures
```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

Infers types if not included, taking the first call to closure. 

### Returning errors

#### go
```go
if err != nil {
	return nil, err
}
```
#### rust
```rust
?
```