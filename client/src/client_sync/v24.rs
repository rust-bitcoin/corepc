// SPDX-License-Identifier: CC0-1.0

//! A JSON-RPC client for testing against Bitcoin Core `v24`.
//!
//! We ignore option arguments unless they effect the shape of the returned JSON data.

use std::collections::BTreeMap;
use std::path::Path;

use bitcoin::address::{Address, NetworkChecked};
use bitcoin::{Amount, Block, BlockHash, PublicKey, Txid};

use crate::client_sync::into_json;
use crate::types::v24::*;

#[rustfmt::skip]                // Keep public re-exports separate.
pub use crate::client_sync::{
    v17::{Input, Output, TemplateRequest, TemplateRules, WalletCreateFundedPsbtInput},
    v23::AddressType,
};

crate::define_jsonrpc_minreq_client!("v24");
crate::impl_client_check_expected_server_version!({ [240200] });

// == Blockchain ==
crate::impl_client_v17__getbestblockhash!();
crate::impl_client_v17__getblock!();
crate::impl_client_v17__getblockchaininfo!();
crate::impl_client_v17__getblockcount!();
crate::impl_client_v19__getblockfilter!();
crate::impl_client_v17__getblockhash!();
crate::impl_client_v17__getblockheader!();
crate::impl_client_v17__getblockstats!();
crate::impl_client_v17__getchaintips!();
crate::impl_client_v17__getchaintxstats!();
crate::impl_client_v17__getdifficulty!();
crate::impl_client_v17__getmempoolancestors!();
crate::impl_client_v17__getmempooldescendants!();
crate::impl_client_v17__getmempoolentry!();
crate::impl_client_v17__getmempoolinfo!();
crate::impl_client_v17__getrawmempool!();
crate::impl_client_v17__gettxout!();
crate::impl_client_v17__gettxoutproof!();
crate::impl_client_v17__gettxoutsetinfo!();
crate::impl_client_v17__preciousblock!();
crate::impl_client_v17__pruneblockchain!();
crate::impl_client_v23__savemempool!();
crate::impl_client_v17__verifychain!();
crate::impl_client_v17__verifytxoutproof!();

// == Control ==
crate::impl_client_v17__getmemoryinfo!();
crate::impl_client_v18__getrpcinfo!();
crate::impl_client_v17__help!();
crate::impl_client_v17__logging!();
crate::impl_client_v17__stop!();
crate::impl_client_v17__uptime!();

// == Generating ==
crate::impl_client_v17__generatetoaddress!();
crate::impl_client_v17__invalidateblock!();

// == Mining ==
crate::impl_client_v17__getblocktemplate!();
crate::impl_client_v17__getmininginfo!();
crate::impl_client_v17__getnetworkhashps!();
crate::impl_client_v17__prioritisetransaction!();
crate::impl_client_v17__submitblock!();

// == Network ==
crate::impl_client_v17__getaddednodeinfo!();
crate::impl_client_v17__getnettotals!();
crate::impl_client_v17__getnetworkinfo!();
crate::impl_client_v18__getnodeaddresses!();
crate::impl_client_v17__getpeerinfo!();

// == Rawtransactions ==
crate::impl_client_v18__analyzepsbt!();
crate::impl_client_v17__combinepsbt!();
crate::impl_client_v17__combinerawtransaction!();
crate::impl_client_v17__converttopsbt!();
crate::impl_client_v17__createpsbt!();
crate::impl_client_v17__createrawtransaction!();
crate::impl_client_v17__decodepsbt!();
crate::impl_client_v17__decoderawtransaction!();
crate::impl_client_v17__decodescript!();
crate::impl_client_v17__finalizepsbt!();
crate::impl_client_v17__fundrawtransaction!();
crate::impl_client_v17__getrawtransaction!();
crate::impl_client_v18__joinpsbts!();
crate::impl_client_v17__sendrawtransaction!();
crate::impl_client_v17__signrawtransaction!();
crate::impl_client_v17__signrawtransactionwithkey!();
crate::impl_client_v17__testmempoolaccept!();
crate::impl_client_v18__utxoupdatepsbt!();

// == Wallet ==
crate::impl_client_v17__addmultisigaddress!();
crate::impl_client_v17__bumpfee!();
crate::impl_client_v23__createwallet!();
crate::impl_client_v17__dumpprivkey!();
crate::impl_client_v17__dumpwallet!();
crate::impl_client_v17__getaddressesbylabel!();
crate::impl_client_v17__getaddressinfo!();
crate::impl_client_v17__getbalance!();
crate::impl_client_v19__getbalances!();
crate::impl_client_v17__getnewaddress!();
crate::impl_client_v17__getrawchangeaddress!();
crate::impl_client_v17__getreceivedbyaddress!();
crate::impl_client_v17__gettransaction!();
crate::impl_client_v17__getunconfirmedbalance!();
crate::impl_client_v17__getwalletinfo!();
crate::impl_client_v17__listaddressgroupings!();
crate::impl_client_v17__listlabels!();
crate::impl_client_v17__listlockunspent!();
crate::impl_client_v17__listreceivedbyaddress!();
crate::impl_client_v17__listsinceblock!();
crate::impl_client_v17__listtransactions!();
crate::impl_client_v17__listunspent!();
crate::impl_client_v17__listwallets!();
crate::impl_client_v22__loadwallet!();
crate::impl_client_v17__rescanblockchain!();
crate::impl_client_v17__sendmany!();
crate::impl_client_v17__sendtoaddress!();
crate::impl_client_v17__signmessage!();
crate::impl_client_v17__signrawtransactionwithwallet!();
crate::impl_client_v21__unloadwallet!();
crate::impl_client_v17__walletcreatefundedpsbt!();
crate::impl_client_v17__walletprocesspsbt!();
