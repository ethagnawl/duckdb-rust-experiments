# DuckDB - Rust client experiments

This repository was the sandbox I used when writing my blog post about DuckDB: [A Dab of Duck](https://peterdohertys.website/blog-posts/dab-of-duck.html)

## Setup
This assumes you have already have Rust and Cargo installed.

See: https://doc.rust-lang.org/cargo/getting-started/installation.html

## Run
### Simple
- You can run the application as-is, with a minimal data set using: `cargo run`

### Moar Data
- If you want to get a better feel for how DuckDB handles a non-trivial amount of data, you can:
    - run the populate script: `pushd data && ./populate.sh && popd`
    - create a release build of the sample application: `cargo build -r`
    - run the release build: `./target/release/duck-db-experiments`
