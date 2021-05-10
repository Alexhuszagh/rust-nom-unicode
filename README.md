nom-unicode
===========

[![Build Status](https://api.travis-ci.org/Alexhuszagh/rust-nom-unicode.svg?branch=master)](https://travis-ci.org/Alexhuszagh/rust-nom-unicode)
[![Latest Version](https://img.shields.io/crates/v/nom-unicode.svg)](https://crates.io/crates/nom-unicode)
[![Rustc Version 1.31+](https://img.shields.io/badge/rustc-1.31+-lightgray.svg)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)

Unicode primitive parsing extensions for [nom](https://github.com/Geal/nom).

**Table of Contents**

- [Getting Started](#getting-started)
- [License](#license)
- [Contributing](#contributing)

# Getting Started

Add nom-unicode to your `Cargo.toml`:

```toml
[dependencies]
nom-unicode = "^0.2"
```

And get started using nom-unicode:

```rust
extern crate nom;
extern crate nom_unicode;

fn alpha0(i: &str) -> nom::IResult<&str, &str> {
    nom_unicode::complete::alpha0(i)
}

fn main() {
    println!("{:?}", alpha0("hello"));
    println!("{:?}", alpha0("erfüllen"));
    println!("{:?}", alpha0("안녕 잘 지내?"));
}
```

# Minimum Standard Required Version

The minimum, standard, required version for nom-unicode will be the same as nom. As of nom-6, it is currently 1.43.0.

# License

Nom-Unicode is dual licensed under the Apache 2.0 license as well as the MIT license. See the LICENCE-MIT and the LICENCE-APACHE files for the licenses.

# Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in nom-unicode by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
