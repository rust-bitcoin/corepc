# bitreq - forked from minreq
[![Crates.io](https://img.shields.io/crates/d/bitreq.svg)](https://crates.io/crates/bitreq)
[![Documentation](https://docs.rs/bitreq/badge.svg)](https://docs.rs/bitreq)

This crate is a fork for the very nice
[minreq](https://github.com/neonmoe/minreq). I chose to fork and
rename it because I wanted to totally gut it and provide a crate with
different goals. Many thanks to the original author.

Simple, minimal-dependency HTTP client. Optional features for
unicode domains (`punycode`), http proxies (`proxy`), async support
(`async`, `async-https`), and https with various TLS implementations
(`https-rustls`, `https-rustls-probe`, and `https` which is an alias
for `https-rustls`).

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

## Planned for 3.0.0

This is a list of features I'll implement once it gets long enough, or
a severe enough issue is found that there's good reason to make a
major version bump.

- Change the response/request structs to allow multiple headers with
  the same name.
- Set sane defaults for maximum header size and status line
  length. The ability to add maximums was added in response to
  [#55](https://github.com/neonmoe/minreq/issues/55), but defaults for
  the limits is a breaking change.
- Clearer error when making a request to an url that does not start
  with `http://` or `https://`.
- Change default proxy port to 1080 (from 8080). Curl uses 1080, so it's a sane
  default.
- Bump MSRV enough to compile the latest versions of all dependencies, and add
  the `rust-version` (at least 1.56) and `edition` (at least 2021) fields to
  Cargo.toml.

### Potential ideas

Just thinking out loud, might not end up doing some or all of these.

- Non-exhaustive error type, to be able to add new errors in minor
  versions.
- Refactor applicable parts to `#![no_std]`, maybe even exposing a
  less convenient API for `#![no_std]` usage. Keep the current API as
  in any case (at the very least, as a default feature).
  - Maybe something along the lines of ["The case for
    sans-io"](https://fasterthanli.me/articles/the-case-for-sans-io)?
    Adding the much-requested async support as a feature could be
    pretty clean if built around this idea.
- Would be good if the crate got smaller with 3.0, not bigger. Maybe
  there's something to cut, something to optimize?
