
<div align="center">

# pino_enum_string
Derive macro for Deref and DerefMut

[![crates.io](https://img.shields.io/crates/v/pino_enum_string.svg)](https://crates.io/crates/pino_enum_string)
[![docs.rs](https://docs.rs/pino_enum_string/badge.svg)](https://docs.rs/pino_enum_string)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

Example usage
```rust
use pino_enum_string::enum_string;

#[enum_string]
enum Weapon {
    Red,
    Blue,
    Green,
}

fn main() {
    assert_eq!("Red", Weapon::Red.to_string());
    assert_eq!("Blue", Weapon::Blue.to_string());
    assert_eq!("Green", Weapon::Green.to_string());
}
```
