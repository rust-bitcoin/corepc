# 0.5.0 - 2024-12-06

- Rename `BitcoinD` to `Node`
- Add support for Bitcoin Core v27.2

# 0.4.0 - 2024-11-14

- Add support for Bitcoin Core v28
- Re-name the repository from `rust-bitcoind-josn-rpc` to `corepc`.
- Re-name the crate from `bitcoind-josn-rpc-regtest` to `corepc-node`.

# 0.3.0 - 2024-06-21

- Call `into_model` when creating/loading wallet.

# 0.2.1 - 2024-06-17

Do various little fixes to try and make the docs on `Client` more legible, specifically to alleviate
confusion around the flag on `docs.rs` that says "Available on crate feature 26_0 only."

# 0.2.0 - 2024-06-13

- Use Bitcoin Core 0.17.1 (0.17.2 seems to not exist and have been a mistake).

# 0.1.0 - 2024-06-13

Initial release, this is an import of `bitcoind v.036.0`.

