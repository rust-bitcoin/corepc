# 0.41.0 - 2026-06-18

- Fix timeout for old versions [#617](https://github.com/rust-bitcoin/corepc/pull/617)
- Upgrade to latest version of `corepc-types`, `corepc-client`, and `bitcoind`

# 0.40.0 - 2026-05-26

With this release we now set the default feature to `electrs_0_10_6`.

- Include in workspace and CI [#570](https://github.com/rust-bitcoin/corepc/pull/570)
- Upgrade to latest version of `corepc-types`, `corepc-client`, and `bitcoind`

# 0.39.0 - 2026-05-12

- Update to use latest `bitcoind v0.39.0` and `corepc-client v0.14.0`.
- Fix the `bitcoind_23_2` feature to select the matching `bitcoind` feature [#573](https://github.com/rust-bitcoin/corepc/pull/573)

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
