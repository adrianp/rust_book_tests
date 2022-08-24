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
* To format the code: `cargo fmt`
* To check the code for issues: `cargo clippy` or `cargo clippy -- -A clippy::all` (also see [docs](https://github.com/rust-lang/rust-clippy#clippy))

The Rust module system, includes:
* Modules and use: Let you control the organization, scope, and privacy of paths
* Crates: A tree of modules that produces a library or executable (we have binary and library crates)
* Packages: A Cargo feature that lets you build, test, and share crates
* Paths: A way of naming an item, such as a struct, function, or module

Other resources:
* [Rust Api Guidelines](https://rust-lang.github.io/api-guidelines/about.html)
* [Rust crate registry](https://crates.io/)
