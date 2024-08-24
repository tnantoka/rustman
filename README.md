# Rustman

 A tiny static site generator for logging my Rust TIL (Today I Learned).

## Demo

https://rustman.tnantoka.com/

## Usage

```sh
# Build the project
$ cargo build

# Create a new site
$ ./target/debug/rustman new hello

# Build the site
$ cd hello
$ ../target/debug/rustman build

# Serve with any static file server
$ serve build
```