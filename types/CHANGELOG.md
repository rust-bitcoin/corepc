# 0.10.1 - 2025-10-10

- v24+ should use the correct `GetRawMempoolVerbose` [#381](https://github.com/rust-bitcoin/corepc/pull/381)

# 0.10.0 - 2025-10-07

- Add `ScriptPubKey` model [#370](https://github.com/rust-bitcoin/corepc/pull/370)
- Add a feature for `serde` deny unknown fields [#367](https://github.com/rust-bitcoin/corepc/pull/367)
- Fix a few of type fields in v24 and v28 fixes [#375](https://github.com/rust-bitcoin/corepc/pull/375)

# 0.9.0 - 2025-09-11

This release is massive, it delivers support for **all** documented Core RPC
methods. It also adds integration testing for all the new ones and many that
were previously untested.

Props to Jamil Lambert for grinding this out.

There are a set of undocumented methods that we will be adding support
for also shortly, stay tuned.

- Implement all remaining non-hidden RPC methods.
- Integration test all methods (excl. two that have open issues).
- Add support for Core `v28.2` [#279](https://github.com/rust-bitcoin/corepc/pull/279)

# 0.8.0 - 2025-05-21

- doc: update docs for now explicit download feature flag [#177](https://github.com/rust-bitcoin/corepc/pull/177)
- Implement all v17 util functions [#163](https://github.com/rust-bitcoin/corepc/pull/163)
- Add support for Bitcoin Core 29.0 [#131](https://github.com/rust-bitcoin/corepc/pull/131)
- Improve the version specific SSOT docs [#142](https://github.com/rust-bitcoin/corepc/pull/142)
- Remove model for `dumpwallet` [#141](https://github.com/rust-bitcoin/corepc/pull/141)
- Implement `pruneblock` method and test [#132](https://github.com/rust-bitcoin/corepc/pull/132)
- Implement `savemempool` method and test [#148](https://github.com/rust-bitcoin/corepc/pull/148)
- Implement `verifychain` method and test [#155](https://github.com/rust-bitcoin/corepc/pull/155)
- Implement `getnodeaddresses` method and test [#154](https://github.com/rust-bitcoin/corepc/pull/154)
- Change `signmessage` returned signature type [#179](https://github.com/rust-bitcoin/corepc/pull/179)
- Add model for `getnodeaddresses` [#191](https://github.com/rust-bitcoin/corepc/pull/191)

# 0.7.0 - 2025-04-04

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
- Re-name the repository from `rust-bitcoind-json-rpc` to `corepc`.
- Re-name the crate from `bitcoind-json-rpc-types` to `corepc-types`.

# 0.3.0 - 2024-06-21

- Implement `into_model` on all types.

# 0.2.0 - 2024-06-13

- Use Bitcoin Core 0.17.1 (0.17.2 seems to not exist and have been a mistake).
- Fix `GetTransactionDetail` conversion to use a signed bitcoin amount.

# 0.1.0 - 2024-06-13

Initial release.
