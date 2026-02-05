// SPDX-License-Identifier: CC0-1.0

//! An async JSON-RPC client for Bitcoin Core `v28`.

use bitcoin::{Block, BlockHash, Txid};

use crate::client_async::into_json;
use crate::types::v28::*;

crate::define_jsonrpc_bitreq_async_client!("v28");
crate::impl_async_client_check_expected_server_version!({ [280000, 280100, 280200] });

// == Blockchain ==
crate::impl_async_client_v17__get_block!();
crate::impl_async_client_v17__get_block_count!();
crate::impl_async_client_v19__get_block_filter!();
crate::impl_async_client_v17__get_block_hash!();
crate::impl_async_client_v17__get_block_header!();
crate::impl_async_client_v21__get_raw_mempool!();

// == Network ==
crate::impl_async_client_v17__get_network_info!();

// == Rawtransactions ==
crate::impl_async_client_v17__get_raw_transaction!();
