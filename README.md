# A Simple Rust Redirector / URL Shortener
A simple url shortener using rust and actix_web. It fetches urls table from redirector database and returns redirect response accordingly (or throws 404 Page Not Found error if it doesn't exist).
## Requirements
- Rust (obviously)
And, that's it.

## Run
To run it, just run `cargo run`. Or, build it in release mode using `cargo build --release`. Then `cd target/release/` and there it is. Your executable!
