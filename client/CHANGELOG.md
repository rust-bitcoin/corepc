# 0.10.0 - 2025-10-07

- Update to use latest `types v0.10.0`.

# 0.9.0 - 2025-09-11

Add support for all the new methods added as part of the `types v0.9.0`
release - that  means **all** of the documented Core RPC methods.

- Implement all remaining non-hidden RPC methods.
- Integration test all methods (excl. two that have open issues).
- Add support for Core `v28.2` [#279](https://github.com/rust-bitcoin/corepc/pull/279)

# 0.8.0 - 2025-05-21

- Add support for Bitcoin Core 29.0 [#131](https://github.com/rust-bitcoin/corepc/pull/131)
- Add support for Bitcoin Core 28.1 [#184](https://github.com/rust-bitcoin/corepc/pull/184)
- Add support for Bitcoin Core 0.17.2 [#128](https://github.com/rust-bitcoin/corepc/pull/128)
- Remove unnecessary error variants [#127](https://github.com/rust-bitcoin/corepc/pull/127)
- Move types to version specific module [#156](https://github.com/rust-bitcoin/corepc/pull/156)
- Move `TemplateRequest` and `TemplateRules` into their proper module [#151](https://github.com/rust-bitcoin/corepc/pull/151)
- Implement `pruneblock` method and test [#132](https://github.com/rust-bitcoin/corepc/pull/132)
- Implement `savemempool` method and test [#148](https://github.com/rust-bitcoin/corepc/pull/148)
- Implement `verifychain` method and test [#155](https://github.com/rust-bitcoin/corepc/pull/155)
- Implement `getnodeaddresses` method and test [#154](https://github.com/rust-bitcoin/corepc/pull/154)

# 0.7.0 - 2025-04-04

- Fix unloadwallet method [#110](https://github.com/rust-bitcoin/corepc/pull/110)
- Implement methods from the mining section [#106](https://github.com/rust-bitcoin/corepc/pull/106)

# 0.6.1 - 2025-13-13

- Enable `std` feature in `types` crate [#98](https://github.com/rust-bitcoin/corepc/pull/98)

# 0.6.0 - 2025-03-07

- Expose all methods from `blockchain` section [#79](https://github.com/rust-bitcoin/corepc/pull/79)
- Fix bugs in tx out proof methods
- Improve docs
- Reduce number of Core minor versions supported

# 0.5.0 - 2024-12-16

- Add support for Bitcoin Core v27.2
- Add a bunch more methods to the `Client`
- Flesh out v17 and v18

# 0.4.0 - 2024-11-14

- Add support for Bitcoin Core v28
- Re-name the repository from `rust-bitcoind-json-rpc` to `corepc`.
- Re-name the crate from `bitcoind-json-rpc-client` to `corepc-client`.

# 0.3.0 - 2024-06-21

- Fix bugs in `AddressType`

# 0.2.1 - 2024-06-17

- Enable all features in docs build.
- Fix typos in crate level docs.

# 0.2.0 - 2024-06-13

- Use Bitcoin Core 0.17.1 (0.17.2 seems to not exist and have been a mistake).

# 0.1.0 - 2024-06-13

Initial release.
