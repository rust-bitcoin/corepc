# Bitcoind

Utility to run a regtest bitcoind process, useful in integration testing environment.

When the auto-download feature is enabled, starting a regtest node is as simple as that:

```rust
// the download feature must be enabled with a specific version, for example `25_1` or `24_0_1`
#[cfg(feature = "download")]
{
  let node = bitcoind::BitcoinD::from_downloaded().unwrap();
  assert_eq!(0, node.client.get_blockchain_info().unwrap().blocks);
}
```

The build script will automatically download the bitcoin core version 25.1 from [bitcoin core](https://bitcoincore.org),
verify the binary hash and place it in the build directory for this crate.

When you don't use the auto-download feature you have the following options:

* have `bitcoind` executable in the `PATH`
* provide the `bitcoind` executable via the `BITCOIND_EXE` env var

```rust
if let Ok(exe_path) = bitcoind::exe_path() {
  let node = bitcoind::BitcoinD::new(exe_path).unwrap();
  assert_eq!(0, node.client.get_blockchain_info().unwrap().blocks);
}
```

Startup options could be configured via the [`Conf`] struct using [`BitcoinD::with_conf`] or
`BitcoinD::from_downloaded_with_conf`

## Features

  * Waits until bitcoind daemon becomes ready to accept RPC commands
  * `bitcoind` uses a temporary directory as datadir. You can specify the root of your temp
    directories so that you have the node's datadir in a RAM disk (eg `/dev/shm`)
  * Free ports are requested from the OS. Since you can't reserve the given port, a low probability
    race condition is still possible, for this reason the process attempts spawning 3 times with
    different ports.
  * The process is killed when the struct goes out of scope no matter how the test finishes.
  * Allows easy spawning of dependent processes like:
    - [electrs](https://github.com/RCasatta/electrsd)
    - [cln](https://github.com/RCasatta/lightningd)
    - [elements](https://github.com/RCasatta/elementsd)

Thanks to these features every `#[test]` could easily run isolated with its own environment.

## Doc

To build docs:

```sh
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --features download,doc --open
```

## Minimum Supported Rust Version (MSRV)

This library should always compile with any combination of features on **Rust 1.75.0**.

## Nix

For reproducibility reasons, Nix build scripts cannot hit the internet, but the auto-download
feature does exactly that. To successfully build under Nix the user must provide the tarball locally
and specify its location via the `BITCOIND_TARBALL_FILE` env var.

Another option is to specify the `BITCOIND_SKIP_DOWNLOAD` env var and provide the executable via the
`PATH`.

Alternatively, use the dep without the auto-download feature.
