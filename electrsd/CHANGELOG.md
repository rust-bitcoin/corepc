# 0.38.1 - 2026-05-01

- Fix typo in manifest `documentation` field.
- Fix stale documented way to depend on this crate.

# 0.38.0 - 2026-04-20

- Update to latest `bitcoind v0.38.0`, optional `bitreq v0.3.5`, and
  `corepc-client v0.13.0`.
- Separate Bitcoin Core binary downloading into a dedicated `bitcoind_download`
  feature [#559](https://github.com/rust-bitcoin/corepc/pull/559).

# 0.37.0 - 2026-04-16

- Import `electrsd` into the `corepc` repository [#542](https://github.com/rust-bitcoin/corepc/pull/542)
- Add support for Bitcoin Core 30.2 [#542](https://github.com/rust-bitcoin/corepc/pull/542)
- Update to `corepc-client v0.12.0` and the restored `bitcoind v0.37.0` crate [#542](https://github.com/rust-bitcoin/corepc/pull/542)
