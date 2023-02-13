<div align="center">
  <h1>bit-set</h1>
  <p>
    <strong>A compact set of bits.</strong>
  </p>
  <p>

[![crates.io][crates.io shield]][crates.io link]
[![Documentation][docs.rs badge]][docs.rs link]
![Rust CI][github ci badge]
[![rustc 1.0+]][Rust 1.0]
[![serde_derive: rustc 1.31+]][Rust 1.31]
<br />
<br />
[![Dependency Status][deps.rs status]][deps.rs link]
[![Download Status][shields.io download count]][crates.io link]

  </p>
</div>

[crates.io shield]: https://img.shields.io/crates/v/bit-set?label=latest
[crates.io link]: https://crates.io/crates/bit-set
[docs.rs badge]: https://docs.rs/bit-set/badge.svg?version=0.5.3
[docs.rs link]: https://docs.rs/bit-set/0.5.3/bit_set/
[github ci badge]: https://github.com/contain-rs/linked-hash-map/workflows/Rust/badge.svg?branch=master
[rustc 1.0+]: https://img.shields.io/badge/rustc-1.0%2B-blue.svg
[Rust 1.0]: https://blog.rust-lang.org/2015/05/15/Rust-1.0.html
[deps.rs status]: https://deps.rs/crate/bit-set/0.5.3/status.svg
[deps.rs link]: https://deps.rs/crate/bit-set/0.5.3
[shields.io download count]: https://img.shields.io/crates/d/bit-set.svg

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
bit-set = "0.5"
```

Since Rust 2018, `extern crate` is no longer mandatory. If your edition is old (Rust 2015),
add this to your crate root:

```rust
extern crate bit_set;
```

If you want to use bit-set in a program that has `#![no_std]`, just drop default features:

```toml
[dependencies]
bit-set = { version = "0.5", default-features = false }
```

<!-- cargo-rdme -->

## License

Dual-licensed for compatibility with the Rust project.

Licensed under the Apache License Version 2.0: http://www.apache.org/licenses/LICENSE-2.0,
or the MIT license: http://opensource.org/licenses/MIT, at your option.
