> [!IMPORTANT]
> **This repository has moved.** Active development is now on rust-bitcoin Forgejo:
>
> ### [git.rust-bitcoin.org/rust-bitcoin/corepc](https://git.rust-bitcoin.org/rust-bitcoin/corepc)
>
> This GitHub repository is **no longer maintained here**.
> Please open all issues and pull requests on the new site.

# corepc-client

Rust client for the Bitcoin Core daemon's JSON-RPC API.

This crate provides:

- A blocking client intended for integration testing (`client-sync`).
- An async client intended for production (`client-async`).

## Features

- `client-sync`: Blocking JSON-RPC client.
- `client-async`: Async JSON-RPC client.

## Minimum Supported Rust Version (MSRV)

This library should always compile with any combination of features on **Rust 1.75.0**.

## Licensing

The code in this project is licensed under the [Creative Commons CC0 1.0 Universal license](LICENSE).
We use the [SPDX license list](https://spdx.org/licenses/) and [SPDX IDs](https://spdx.dev/ids/).
