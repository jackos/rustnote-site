# Speed up compiler
## Incremental builds

Only do this locally, `sccache` won't play nice with it in CI
### cargo.toml
```toml
[profile.release]
debug = 1
incremental = true
```