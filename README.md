# systine

[<img src="https://img.shields.io/crates/v/systine.svg?style=flat-square" alt="crates.io link">](https://crates.io/crates/systine)
[<img src="https://docs.rs/systine/badge.svg">](https://docs.rs/systine)

Get system information in Rust.

A fork of [`sys-info`](https://github.com/FillZpp/sys-info-rs). Supports Linux, Mac OS X, illumos, Solaris, FreeBSD,
OpenBSD, NetBSD and Windows, though note this fork will primarily focus on illumos, Solaris, and OpenBSD for use with
[bottom](https://github.com/ClementTsang/bottom). Thanks to all who worked on the original crate!

## Usage

Add this to `Cargo.toml`:

```toml
[dependencies]
systine = "0.10.0"
```

## Licensing

`systine` is licensed under MIT.
