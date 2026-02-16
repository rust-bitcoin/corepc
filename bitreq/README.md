# bitreq - forked from minreq
[![Crates.io](https://img.shields.io/crates/d/bitreq.svg)](https://crates.io/crates/bitreq)
[![Documentation](https://docs.rs/bitreq/badge.svg)](https://docs.rs/bitreq)

This crate is a fork for the very nice
[minreq](https://github.com/neonmoe/minreq). I chose to fork and
rename it because I wanted to totally gut it and provide a crate with
different goals. Many thanks to the original author.

Simple, minimal-dependency HTTP client. Optional features for http
proxies (`proxy`), async support (`async`, `async-https`), and https
with various TLS implementations (`https-rustls`, `https-rustls-probe`,
and `https` which is an alias for `https-rustls`).

Without any optional features, my casual testing indicates about 100
KB additional executable size for stripped release builds using this
crate. Compiled with rustc 1.45.2, `println!("Hello, World!");` is 239
KB on my machine, where the [hello](examples/hello.rs) example is 347
KB. Both are pure Rust, so aside from `libc`, everything is statically
linked.

Note: some of the dependencies of this crate (especially the various
`https` libraries) are a lot more complicated than this library, and
their impact on executable size reflects that.

## Documentation

Build your own with `cargo doc --all-features`, or browse the online
documentation at [docs.rs/bitreq](https://docs.rs/bitreq).

## Minimum Supported Rust Version (MSRV)

If you don't care about the MSRV, you can ignore this section
entirely, including the commands instructed.

We use an MSRV per major release, i.e., with a new major release we
reserve the right to change the MSRV.

The current major version of this library should always compile with
default features (i.e., `std`) on **Rust 1.75**. Other features may
require a higher MSRV.

## License
This crate is distributed under the terms of the [ISC license](COPYING.md).
