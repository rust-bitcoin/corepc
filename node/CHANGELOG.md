# 0.10.1 2025-11-18

- Remove `doc_auto_cfg` to fix build on docs.rs

# 0.10.0 2025-10-07

- Update to use latest `client v0.10.0`.

# 0.9.0 2025-09-11

The `types v0.9.0` release adds support for **all** remaining documented
Core RPC methods. These are then pickup up in the update of `client`.

- Update to use the new `client v0.9.0`.
- Fix race condition in node start up [#213](https://github.com/rust-bitcoin/corepc/pull/213)
- Fix build on macOS. Only codesign bitcoind if necessary [#309](https://github.com/rust-bitcoin/corepc/pull/309)
- Add support for Core `v28.2` [#279](https://github.com/rust-bitcoin/corepc/pull/279)

# 0.8.0 2025-05-21

- Add support for Bitcoin Core 29.0 [#131](https://github.com/rust-bitcoin/corepc/pull/131)
- Add support for Core version 28.1 [#184](https://github.com/rust-bitcoin/corepc/pull/184)
- Add support for Bitcoin Core 0.17.2 [#128](https://github.com/rust-bitcoin/corepc/pull/128)
- Upgrade `zip` in light of RUSTSEC-2020-0071 [#143](https://github.com/rust-bitcoin/corepc/pull/143)
- Drop default features for `zip` [#130](https://github.com/rust-bitcoin/corepc/pull/130)

# 0.7.1 2025-05-05

- backport: bump zip in light of RUSTSEC-2020-0071 [#145](https://github.com/rust-bitcoin/corepc/pull/145)

# 0.7.0 2025-04-04

- Retry initial client connections [#111](https://github.com/rust-bitcoin/corepc/pull/111)

# 0.6.1 - 2025-03-11

- Fix the docs.rs build [#92](https://github.com/rust-bitcoin/corepc/pull/92)

# 0.6.0 - 2025-03-07

- Remove `default` feature [#45](https://github.com/rust-bitcoin/corepc/pull/45)
- Reduce number of supported minor versions [#27](https://github.com/rust-bitcoin/corepc/pull/27)
- Enable running multiple Core v28 nodes [#46](https://github.com/rust-bitcoin/corepc/pull/46)

# 0.5.0 - 2024-12-16

- Rename `BitcoinD` to `Node`
- Add support for Bitcoin Core v27.2

# 0.4.0 - 2024-11-14

- Add support for Bitcoin Core v28
- Re-name the repository from `rust-bitcoind-json-rpc` to `corepc`.
- Re-name the crate from `bitcoind-json-rpc-regtest` to `corepc-node`.

# 0.3.0 - 2024-06-21

- Call `into_model` when creating/loading wallet.

# 0.2.1 - 2024-06-17

Do various little fixes to try and make the docs on `Client` more legible, specifically to alleviate
confusion around the flag on `docs.rs` that says "Available on crate feature 26_0 only."

# 0.2.0 - 2024-06-13

- Use Bitcoin Core 0.17.1 (0.17.2 seems to not exist and have been a mistake).

# 0.1.0 - 2024-06-13

Initial release, this is an import of `bitcoind v.036.0`.
