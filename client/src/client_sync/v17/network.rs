// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Requires `Client` to be in scope.
//!
//! Specifically this is methods found under the `== Network ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! See, or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `addnode`.
#[macro_export]
macro_rules! impl_client_v17__add_node {
    () => {
        impl Client {
            pub fn add_node(&self, node: &str, command: AddNodeCommand) -> Result<()> {
                match self.call("addnode", &[into_json(node)?, into_json(command)?]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `clearbanned`.
#[macro_export]
macro_rules! impl_client_v17__clear_banned {
    () => {
        impl Client {
            pub fn clear_banned(&self) -> Result<()> {
                match self.call("clearbanned", &[]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `disconnectnode`.
#[macro_export]
macro_rules! impl_client_v17__disconnect_node {
    () => {
        impl Client {
            pub fn disconnect_node(&self, address: &str) -> Result<()> {
                match self.call("disconnectnode", &[into_json(address)?]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getaddednodeinfo`.
#[macro_export]
macro_rules! impl_client_v17__get_added_node_info {
    () => {
        impl Client {
            pub fn get_added_node_info(&self) -> Result<GetAddedNodeInfo> {
                self.call("getaddednodeinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getconnectioncount`.
#[macro_export]
macro_rules! impl_client_v17__get_connection_count {
    () => {
        impl Client {
            pub fn get_connection_count(&self) -> Result<GetConnectionCount> {
                self.call("getconnectioncount", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getnettotals`.
#[macro_export]
macro_rules! impl_client_v17__get_net_totals {
    () => {
        impl Client {
            pub fn get_net_totals(&self) -> Result<GetNetTotals> { self.call("getnettotals", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getnetworkinfo`.
#[macro_export]
macro_rules! impl_client_v17__get_network_info {
    () => {
        impl Client {
            /// Returns the server version field of `GetNetworkInfo`.
            pub fn server_version(&self) -> Result<usize> {
                let info = self.get_network_info()?;
                Ok(info.version)
            }

            pub fn get_network_info(&self) -> Result<GetNetworkInfo> {
                self.call("getnetworkinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getpeerinfo`.
#[macro_export]
macro_rules! impl_client_v17__get_peer_info {
    () => {
        impl Client {
            pub fn get_peer_info(&self) -> Result<GetPeerInfo> { self.call("getpeerinfo", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listbanned`.
#[macro_export]
macro_rules! impl_client_v17__list_banned {
    () => {
        impl Client {
            pub fn list_banned(&self) -> Result<ListBanned> { self.call("listbanned", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `ping`.
#[macro_export]
macro_rules! impl_client_v17__ping {
    () => {
        impl Client {
            pub fn ping(&self) -> Result<()> {
                match self.call("ping", &[]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `setban`.
#[macro_export]
macro_rules! impl_client_v17__set_ban {
    () => {
        impl Client {
            pub fn set_ban(&self, subnet: &str, command: SetBanCommand) -> Result<()> {
                match self.call("setban", &[into_json(subnet)?, into_json(command)?]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `setnetworkactive`.
#[macro_export]
macro_rules! impl_client_v17__set_network_active {
    () => {
        impl Client {
            pub fn set_network_active(&self, state: bool) -> Result<SetNetworkActive> {
                self.call("setnetworkactive", &[into_json(state)?])
            }
        }
    };
}
