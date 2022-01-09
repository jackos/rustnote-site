# Rustnote
My personal notes for Rust built with [mdbook](https://rust-lang.github.io/mdBook) 

The different things I've done with Rust to produce these notes:

### Books
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Zero to Prod in Rust](https://www.zero2prod.com/index.html)
- [Rust for Rustaceans](https://nostarch.com/rust-rustaceans)

### Projects
- [rustkernel](https://github.com/jackos/rustkernel): Takes in rust code from a VS Code notebook, runs it and returns stdout and stderror so it can be rendered in VS Code, behaves like a jupyter notebook.
- [rustnote](https://github.com/jackos/rustnote): Uses `rustkernel` from VS code to run Rust code interactively inside an .md document
- [ech](https://github.com/jackos/ech): A very simple TCP server to return the message back to the caller in a raw u8 stream, and print it to stdout. It's helpful for programs where you don't have the source code, or are trying to understand what's actually being sent out by a messy program.
- [postport](https://github.com/postport/postport): Converts Postman collections to interactive static websites (still experimental)

### Exercises and Algos
- [Leetcode Rust solutions](https://github.com/jackos/leetcode)
- [Rustlings solutions](https://github.com/jackos/rustlings/tree/jackos/exercises)

### Contributions
- [The Rust Book](https://github.com/rust-lang/book/pull/2895): Made generic type names more idiomatic in the mixup section