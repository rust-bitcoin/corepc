# 0.7.0 2025-04-04

- Fix `{create,load}wallet` on `v25` [#108](https://github.com/rust-bitcoin/corepc/pull/108)
- Fix unloadwallet method [#110](https://github.com/rust-bitcoin/corepc/pull/110)
- Implement methods from the mining section [#106](https://github.com/rust-bitcoin/corepc/pull/106)

# 0.6.1 - 2025-03-11

- Add missing transaction categories [#91](https://github.com/rust-bitcoin/corepc/pull/91)

# 0.6.0 - 2025-03-07

- Add `std` feature [#44](https://github.com/rust-bitcoin/corepc/pull/44)
- Reduce number of supported minor versions [#27](https://github.com/rust-bitcoin/corepc/pull/27)
- Update the version specific docs for Core versions 19 - 28
   - [#55](https://github.com/rust-bitcoin/corepc/pull/55)
   - [#64](https://github.com/rust-bitcoin/corepc/pull/64)

# 0.5.0 - 2024-12-16

- Flesh out v17 and v18
- Re-write docs and verify correctness of status claims

# 0.4.0 - 2024-11-14

- Add support for Bitcoin Core v28
- Re-name the repository from `rust-bitcoind-josn-rpc` to `corepc`.
- Re-name the crate from `bitcoind-josn-rpc-types` to `corepc-types`.

# 0.3.0 - 2024-06-21

- Implement `into_model` on all types.

# 0.2.0 - 2024-06-13

- Use Bitcoin Core 0.17.1 (0.17.2 seems to not exist and have been a mistake).
- Fix `GetTransactionDetail` conversion to use a signed bitcoin amount.

# 0.1.0 - 2024-06-13

Initial release.
