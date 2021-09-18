
## Regex
### Cargo.toml
---
```toml
[dependencies]
regex = "0.1.41"
```
---

### match_all
---
```rust
let re = regex::Regex::new(r"test|regex").expect("Regex problem");
println!("Did test match? {}", re.is_match("test"))
```
---