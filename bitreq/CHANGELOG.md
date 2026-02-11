# 0.3.2 - 2026-02-10

* Fix issues with non-`std` builds and add `#[no_std]` attribute [#498](https://github.com/rust-bitcoin/corepc/pull/498)
* `Url` follow-ups [#491](https://github.com/rust-bitcoin/corepc/pull/491)
* Handle async pipelined request cancellation correctly [#488](https://github.com/rust-bitcoin/corepc/pull/488)
* Check utf-8 while deserializing JSON body [#486](https://github.com/rust-bitcoin/corepc/pull/486)
* Add `Url` type for parsing and validating URLs [#467](https://github.com/rust-bitcoin/corepc/pull/467)
* Expose all features in `docs.rs` builds [#466](https://github.com/rust-bitcoin/corepc/pull/466) 

# 0.3.1 - 2026-01-19

* Add default size limits for headers, status line, and body [#463](https://github.com/rust-bitcoin/corepc/pull/463)

# 0.3.0 - 2026-01-16

* Fix a denial-of-service issue due to lack of bounding in response size [#452](https://github.com/rust-bitcoin/corepc/pull/452)
* Add support for `native-tls` in addition to `rustls` [#451](https://github.com/rust-bitcoin/corepc/pull/451)
* Support connection reuse via a `Client` object [#450](https://github.com/rust-bitcoin/corepc/pull/450)
* Make `async` native async rather than spawning a blocking task [#448](https://github.com/rust-bitcoin/corepc/pull/448)
* Remove `urlencoding` dependence [#424](https://github.com/rust-bitcoin/corepc/pull/424)
* Remove `punycode` dependency [#423](https://github.com/rust-bitcoin/corepc/pull/423)
* Remove unused `tokio` features [#421](https://github.com/rust-bitcoin/corepc/pull/421)

# 0.2.0 - 2025-10-31

* Re-implement `json-using-serde` feature [#398](https://github.com/rust-bitcoin/corepc/pull/398)
* Update MSRV to Rust `v1.75.0` [#405](https://github.com/rust-bitcoin/corepc/pull/405/)

# 0.1.0 - 2025-10-22

* Fork `minreq`, strip it down, and import it into the `corepc` repo.
  Rename to `bitreq` while doing so.

I forked `minreq` from the master branch 3 months ago and did not grab
the commit hash when forking, at the time the released version was 2.13.4
