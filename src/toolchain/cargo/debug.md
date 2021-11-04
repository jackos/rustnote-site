# Debug

## Trace back

To run with extra info about where panic originated outside of the calling code:
```
RUST_BACKTRACE=1 cargo run
```
For even more detailed info:
```
RUST_BACKTRACE=full cargo run
```

Debug symbols will be disabled in `--release` builds, but can be enabled manually.