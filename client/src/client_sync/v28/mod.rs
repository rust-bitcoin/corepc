// SPDX-License-Identifier: CC0-1.0

//! A JSON-RPC client for testing against Bitcoin Core `v28`.
//!
//! We ignore option arguments unless they effect the shape of the returned JSON data.

pub mod raw_transactions;

use bitcoin::address::{Address, NetworkChecked};
use bitcoin::{Amount, Block, BlockHash, Txid};

use crate::client_sync::into_json;
use crate::types::v28::*;

#[rustfmt::skip]                // Keep public re-exports separate.
pub use crate::client_sync::v23::AddressType;

crate::define_jsonrpc_minreq_client!("v28");

// == Blockchain ==
crate::impl_client_v17__getblockchaininfo!();
crate::impl_client_v17__getbestblockhash!();
crate::impl_client_v17__getblock!();
crate::impl_client_v17__gettxout!();

// == Control ==
crate::impl_client_v17__stop!();

// == Generating ==
crate::impl_client_v17__generatetoaddress!();

// == Network ==
crate::impl_client_v17__getnetworkinfo!();
crate::impl_client_check_expected_server_version!({ [280000] });

// == Rawtransactions ==
crate::impl_client_v17__sendrawtransaction!();
crate::impl_client_v28__submitpackage!();

// == Wallet ==
crate::impl_client_v17__createwallet!();
crate::impl_client_v22__unloadwallet!();
crate::impl_client_v22__loadwallet!();
crate::impl_client_v17__getbalance!();
crate::impl_client_v19__getbalances!();
crate::impl_client_v17__getnewaddress!();
crate::impl_client_v17__sendtoaddress!();
crate::impl_client_v17__gettransaction!();
