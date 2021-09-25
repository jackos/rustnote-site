# Testing

## Simple test
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn is_larger() {
        assert!(larger(5, 10) == 10);
    }
}
```

## Test types
All examples would pass
```rust
assert!(larger(5, 10) == 10);
assert_eq!(larger(5, 10), 10);
assert_ne!(larger(5, 10), 5);
```
`assert_eq!` and `assert_ne!` must implement PartialEq and Debug, so structs and enums need to be annotated with:
```rust
#[derive(PartialEq, Debug)]
```