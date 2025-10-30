// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Control ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getmemoryinfo`.
#[macro_export]
macro_rules! impl_client_v17__get_memory_info {
    () => {
        impl Client {
            pub fn get_memory_info(&self) -> Result<GetMemoryInfoStats> {
                self.call("getmemoryinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `help`.
#[macro_export]
macro_rules! impl_client_v17__help {
    () => {
        impl Client {
            pub fn help(&self) -> Result<String> { self.call("help", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `logging`.
#[macro_export]
macro_rules! impl_client_v17__logging {
    () => {
        impl Client {
            pub fn logging(&self) -> Result<Logging> { self.call("logging", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `stop`.
#[macro_export]
macro_rules! impl_client_v17__stop {
    () => {
        impl Client {
            pub fn stop(&self) -> Result<String> { self.call("stop", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `uptime`.
#[macro_export]
macro_rules! impl_client_v17__uptime {
    () => {
        impl Client {
            pub fn uptime(&self) -> Result<u32> { self.call("uptime", &[]) }
        }
    };
}
