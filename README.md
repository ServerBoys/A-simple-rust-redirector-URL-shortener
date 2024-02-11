# A Simple Rust Redirector / URL Shortener
A simple url shortener using rust and actix_web. It fetches urls table from redirector database and returns redirect response accordingly (or throws 404 Page Not Found error if it doesn't exist).
## Requirements
- Rust (obviously)
And, that's it.

## Run
To run it, first make sure you have rust installed. Then, just clone the repo and copy `.env.example` to `.env`. Update the .env file to reflect your server and database. (Note that you have to create your database and add values on that database on your own, this is meant to be a lightweight rust package for a URL shortener). 
Then, run `cargo run` or build it in release mode using `cargo build --release`. Then `cd target/release/` and there it is. Your executable!
