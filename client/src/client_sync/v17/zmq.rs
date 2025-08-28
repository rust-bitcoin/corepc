// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods for the `== Zmq ==` section (v0.17).

/// Implements Bitcoin Core JSON-RPC API method `getzmqnotifications`.
#[macro_export]
macro_rules! impl_client_v17__get_zmq_notifications {
    () => {
        impl Client {
            pub fn get_zmq_notifications(&self) -> Result<Vec<GetZmqNotifications>> {
                self.call("getzmqnotifications", &[])
            }
        }
    };
}
