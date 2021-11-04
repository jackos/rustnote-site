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

## Custom failures
Any arguments after the first `assert!` argument will be passed to `format!`, which will be passed to the panic message. This allows for more detail if a test fails.
```rust
pub fn greeting(name: &str) -> String {
    format!("Hello")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        let name = "Carol";
        assert!(
            result.contains(name),
            "Greeting did not contain name: {}, returned value was: `{}`",
            result,
            name
        );
    }
}
```
## Should panic
Gives a nice expected/got message if it fails, the `expected` parameter is optional for the `should_panic` attribute
```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess > 1 && < 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess > 1 && < 100, got 200")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```
## Return a result
Hey silly, just return a `Result`!
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

## Visibility
`tests folder` and `documentation tests` are compiled in separate binaries that have the same visibility as an external user of a library. To test private objects you must use `embedded tests`.

