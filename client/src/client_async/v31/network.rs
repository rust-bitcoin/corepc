// SPDX-License-Identifier: CC0-1.0

//! Auto-generated method wrappers for Bitcoin Core `31` - network.
//!
//! Produced by `codegen`. Do not edit by hand, re-run
//! `just codegen` to regenerate. Defines the `*Options` request structs these methods
//! consume; the response types live in the `corepc-types` crate
//! (`types::v31::generated`).

#![allow(unused_imports, clippy::needless_pass_by_value, clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use serde_json::json;

use types::v31::generated::{
    GetAddedNodeInfo,
    GetAddrManInfo,
    GetConnectionCount,
    GetNetTotals,
    GetNetworkInfo,
    GetNodeAddresses,
    GetPeerInfo,
    ListBanned,
    SetNetworkActive,
};

use crate::client_async::error::Result;
use crate::client_async::Client;

/// Optional parameters for the `addnode` JSON-RPC method (consumed by `Client::add_node_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddNodeOptions {
    /// Attempt to connect using BIP324 v2 transport protocol (ignored for 'remove' command)
    pub v2transport: Option<bool>,
}

/// Optional parameters for the `disconnectnode` JSON-RPC method (consumed by `Client::disconnect_node_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisconnectNodeOptions {
    /// The IP address/port of the node
    pub address: Option<String>,
    /// The node ID (see getpeerinfo for node IDs)
    pub nodeid: Option<i64>,
}

/// Optional parameters for the `getaddednodeinfo` JSON-RPC method (consumed by `Client::get_added_node_info_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAddedNodeInfoOptions {
    /// If provided, return information about this specific node, otherwise all nodes are returned.
    pub node: Option<String>,
}

/// Optional parameters for the `getnodeaddresses` JSON-RPC method (consumed by `Client::get_node_addresses_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNodeAddressesOptions {
    /// The maximum number of addresses to return. Specify 0 to return all known addresses.
    pub count: Option<i64>,
    /// Return only addresses of the specified network. Can be one of: ipv4, ipv6, onion, i2p, cjdns.
    pub network: Option<String>,
}

/// Optional parameters for the `setban` JSON-RPC method (consumed by `Client::set_ban_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetBanOptions {
    /// time in seconds how long (or until when if \[absolute\] is set) the IP is banned (0 or empty means using the default time of 24h which can also be overwritten by the -bantime startup argument)
    pub ban_time: Option<f64>,
    /// If set, the bantime must be an absolute timestamp expressed in UNIX epoch time
    pub absolute: Option<bool>,
}

impl Client {
    /// `addnode` with required arguments only.
    ///
    /// Attempts to add or remove a node from the addnode list.
    /// Or try a connection to a node once.
    /// Nodes added using addnode (or -connect) are protected from DoS disconnection and are not required to be
    /// full nodes/support SegWit as other outbound peers are (though such peers will not be synced from).
    /// Addnode connections are limited to 8 at a time and are counted separately from the -maxconnections limit.
    pub async fn add_node(&self, node: String, command: String) -> Result<()> {
        self.call_raw("addnode", &[json!(node), json!(command)]).await
    }

    /// `addnode` with all optional arguments via [`AddNodeOptions`].
    ///
    /// Attempts to add or remove a node from the addnode list.
    /// Or try a connection to a node once.
    /// Nodes added using addnode (or -connect) are protected from DoS disconnection and are not required to be
    /// full nodes/support SegWit as other outbound peers are (though such peers will not be synced from).
    /// Addnode connections are limited to 8 at a time and are counted separately from the -maxconnections limit.
    pub async fn add_node_with(&self, node: String, command: String, opts: AddNodeOptions) -> Result<()> {
        self.call_raw("addnode", &[json!(node), json!(command), json!(opts.v2transport)]).await
    }

    /// `clearbanned` with required arguments only.
    ///
    /// Clear all banned IPs.
    pub async fn clear_banned(&self) -> Result<()> {
        self.call_raw("clearbanned", &[(); 0] as &[()]).await
    }

    /// `disconnectnode` with required arguments only.
    ///
    /// Immediately disconnects from the specified peer node.
    /// 
    /// Strictly one out of 'address' and 'nodeid' can be provided to identify the node.
    /// 
    /// To disconnect by nodeid, either set 'address' to the empty string, or call using the named 'nodeid' argument only.
    pub async fn disconnect_node(&self) -> Result<()> {
        self.call_raw("disconnectnode", &[(); 0] as &[()]).await
    }

    /// `disconnectnode` with all optional arguments via [`DisconnectNodeOptions`].
    ///
    /// Immediately disconnects from the specified peer node.
    /// 
    /// Strictly one out of 'address' and 'nodeid' can be provided to identify the node.
    /// 
    /// To disconnect by nodeid, either set 'address' to the empty string, or call using the named 'nodeid' argument only.
    pub async fn disconnect_node_with(&self, opts: DisconnectNodeOptions) -> Result<()> {
        self.call_raw("disconnectnode", &[json!(opts.address), json!(opts.nodeid)]).await
    }

    /// `getaddednodeinfo` with required arguments only.
    ///
    /// Returns information about the given added node, or all added nodes
    /// (note that onetry addnodes are not listed here)
    pub async fn get_added_node_info(&self) -> Result<GetAddedNodeInfo> {
        self.call_raw("getaddednodeinfo", &[(); 0] as &[()]).await
    }

    /// `getaddednodeinfo` with all optional arguments via [`GetAddedNodeInfoOptions`].
    ///
    /// Returns information about the given added node, or all added nodes
    /// (note that onetry addnodes are not listed here)
    pub async fn get_added_node_info_with(&self, opts: GetAddedNodeInfoOptions) -> Result<GetAddedNodeInfo> {
        self.call_raw("getaddednodeinfo", &[json!(opts.node)]).await
    }

    /// `getaddrmaninfo` with required arguments only.
    ///
    /// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
    pub async fn get_addr_man_info(&self) -> Result<GetAddrManInfo> {
        self.call_raw("getaddrmaninfo", &[(); 0] as &[()]).await
    }

    /// `getconnectioncount` with required arguments only.
    ///
    /// Returns the number of connections to other nodes.
    pub async fn get_connection_count(&self) -> Result<GetConnectionCount> {
        self.call_raw("getconnectioncount", &[(); 0] as &[()]).await
    }

    /// `getnettotals` with required arguments only.
    ///
    /// Returns information about network traffic, including bytes in, bytes out,
    /// and current system time.
    pub async fn get_net_totals(&self) -> Result<GetNetTotals> {
        self.call_raw("getnettotals", &[(); 0] as &[()]).await
    }

    /// `getnetworkinfo` with required arguments only.
    ///
    /// Returns an object containing various state info regarding P2P networking.
    pub async fn get_network_info(&self) -> Result<GetNetworkInfo> {
        self.call_raw("getnetworkinfo", &[(); 0] as &[()]).await
    }

    /// `getnodeaddresses` with required arguments only.
    ///
    /// Return known addresses, after filtering for quality and recency.
    /// These can potentially be used to find new peers in the network.
    /// The total number of addresses known to the node may be higher.
    pub async fn get_node_addresses(&self) -> Result<GetNodeAddresses> {
        self.call_raw("getnodeaddresses", &[(); 0] as &[()]).await
    }

    /// `getnodeaddresses` with all optional arguments via [`GetNodeAddressesOptions`].
    ///
    /// Return known addresses, after filtering for quality and recency.
    /// These can potentially be used to find new peers in the network.
    /// The total number of addresses known to the node may be higher.
    pub async fn get_node_addresses_with(&self, opts: GetNodeAddressesOptions) -> Result<GetNodeAddresses> {
        self.call_raw("getnodeaddresses", &[json!(opts.count), json!(opts.network)]).await
    }

    /// `getpeerinfo` with required arguments only.
    ///
    /// Returns data about each connected network peer as a json array of objects.
    pub async fn get_peer_info(&self) -> Result<GetPeerInfo> {
        self.call_raw("getpeerinfo", &[(); 0] as &[()]).await
    }

    /// `listbanned` with required arguments only.
    ///
    /// List all manually banned IPs/Subnets.
    pub async fn list_banned(&self) -> Result<ListBanned> {
        self.call_raw("listbanned", &[(); 0] as &[()]).await
    }

    /// `ping` with required arguments only.
    ///
    /// Requests that a ping be sent to all other nodes, to measure ping time.
    /// Results are provided in getpeerinfo.
    /// Ping command is handled in queue with all other commands, so it measures processing backlog, not just network ping.
    pub async fn ping(&self) -> Result<()> {
        self.call_raw("ping", &[(); 0] as &[()]).await
    }

    /// `setban` with required arguments only.
    ///
    /// Attempts to add or remove an IP/Subnet from the banned list.
    pub async fn set_ban(&self, sub_net: String, command: String) -> Result<()> {
        self.call_raw("setban", &[json!(sub_net), json!(command)]).await
    }

    /// `setban` with all optional arguments via [`SetBanOptions`].
    ///
    /// Attempts to add or remove an IP/Subnet from the banned list.
    pub async fn set_ban_with(&self, sub_net: String, command: String, opts: SetBanOptions) -> Result<()> {
        self.call_raw("setban", &[json!(sub_net), json!(command), json!(opts.ban_time), json!(opts.absolute)]).await
    }

    /// `setnetworkactive` with required arguments only.
    ///
    /// Disable/enable all p2p network activity.
    pub async fn set_network_active(&self, state: bool) -> Result<SetNetworkActive> {
        self.call_raw("setnetworkactive", &[json!(state)]).await
    }

}
