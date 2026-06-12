# corepc-client

Rust clients for the Bitcoin Core daemon's JSON-RPC API.

Two clients live here:

- A blocking client (`client_sync`, the `client-sync` feature) intended for integration testing.
  Its methods are hand-written macros covering Bitcoin Core v17 through v31.
- An async client (`client_async`, the `client-async` feature) intended for production use. Its
  method surface is generated from Bitcoin Core's OpenRPC export by the `codegen/` tool in this
  repository; v30 is the generated version today.

## The async client

```rust,no_run
use corepc_client::client_async::{Auth, Client};

async fn tip_height() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .url("http://127.0.0.1:8332")?
        .auth(Auth::CookieFile("/var/lib/bitcoind/.cookie".into()))
        .build()?;

    let count = client.get_block_count().await?;
    println!("tip height: {}", *count);
    Ok(())
}
```

Every RPC with optional positional arguments gets a required-only method plus a `_with` overload
taking a `*Options` struct (one `Option<T>` field per optional argument); an unset field is sent
as JSON `null`, which Core reads as "use the default". `Client::call_raw` is the escape hatch for
anything without a generated wrapper.

A `blocking` feature adds a sync facade over the async client that exposes the same surface as
the blocking test client, used to run the integration test suite against the async transport.

## Feature flags

- `client-sync`: the blocking test client (per-version modules `v17`..`v31`).
- `client-async`: the generated async client. Requires a version feature; `30_0` (the default
  paired version) is the only generated one today.
- `blocking`: the sync facade over the async client (implies both of the above).

## Minimum Supported Rust Version (MSRV)

This library should always compile with any combination of features on **Rust 1.75.0**.

## Licensing

The code in this project is licensed under the [Creative Commons CC0 1.0 Universal license](LICENSE).
We use the [SPDX license list](https://spdx.org/licenses/) and [SPDX IDs](https://spdx.dev/ids/).
