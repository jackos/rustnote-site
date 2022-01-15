# Rustnote

The content for this website is made using a VS Code extension: [rustnote](https://marketplace.visualstudio.com/items?itemName=jackos.rustnote). It runs Rust code in notebook format, and saves the results to standard markdown (CommonMark).

The site is then built with a static site generator called [mdbook](https://rust-lang.github.io/mdBook).


### My Rust projects
- [rustkernel](https://github.com/jackos/rustkernel): Takes in rust code from a VS Code notebook, runs it and returns stdout and stderror so it can be rendered in VS Code, behaves like a jupyter notebook.
- [rustnote](https://github.com/jackos/rustnote): Uses `rustkernel` from VS code to run Rust code interactively inside an .md document.
- [rustnote-site](https://github.com/jackos/rustnote-site): The source code for this website, including the palenight theme.
- [ech](https://github.com/jackos/ech): A very simple TCP server to return the message back to the caller in a raw u8 stream, and print it to stdout. It's helpful for programs where you don't have the source code, or are trying to understand what's actually being sent out by a messy program.
- [postport](https://github.com/postport/postport): Converts Postman collections to interactive static websites (still experimental).
- [rustlings-fix](https://github.com/jackos/rustlings-fix): Fixes rustlings to work with rust-analyzer (language server). Pull request open for the functionality natively in `rustlings` here: [rustlings](https://github.com/rust-lang/rustlings/pull/911)

### Recommended Books
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Zero to Prod in Rust](https://www.zero2prod.com/index.html)
- [Rust for Rustaceans](https://nostarch.com/rust-rustaceans)