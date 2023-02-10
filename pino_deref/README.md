
<div align="center">

# pino_deref
Derive macro for Deref and DerefMut

[![crates.io](https://img.shields.io/crates/v/pino_deref.svg)](https://crates.io/crates/pino_deref)
[![docs.rs](https://docs.rs/pino_deref/badge.svg)](https://docs.rs/pino_deref)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

Example usage
```rust
use pino_deref::{Deref, DerefMut};

#[derive(Deref, DerefMut)]
struct Nametag(pub String);

fn main() {
    let nametag = Nametag("pinosaur".into());
    assert_eq!(*nametag, String::from("pinosaur"));
}
```
