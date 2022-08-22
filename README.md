My run through the [Rust book](https://doc.rust-lang.org/book/)

Some useful commands:
* To create a new project: `cargo new some_rust_project`
* To check the current project: `cargo check`
* To build the current project: `cargo build` (`--release` will build with optimizations)
* To run current project: `cargo run`; if multiple targets are available you'll need to add the [default-run manifest key](https://doc.rust-lang.org/cargo/reference/manifest.html#the-default-run-field)
* To run a specific file:`cargo run --bin structs`
* To compile a specific file: `rustc src/structs.rs -o main`
* To update Rust: `rustup update`
* To generate local documentation: `rustup doc`
