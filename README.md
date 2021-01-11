# Stovetop

A simple file templating system written in Rust.

## Template Files

Templates can be named anything as long as they are in the [`toml`](https://toml.io) format.
e.g:

```toml
name = "Neo"
occupation = "The One"
```

## Usage
## As A Library
```rust
use stovetop;

fn main() {
    stovetop::generate(
        "./example/template",
        "./example/stovetop.toml",
        "./output",
        None
    ).unwrap();
}

```