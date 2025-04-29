// SPDX-License-Identifier: CC0-1.0

//! A JSON-RPC client for testing against Bitcoin Core `v0.17`.
//!
//! We ignore option arguments unless they effect the shape of the returned JSON data.

pub mod blockchain;
pub mod control;
pub mod generating;
pub mod mining;
pub mod network;
pub mod raw_transactions;
pub mod wallet;

use std::collections::BTreeMap;
use std::path::Path;

use bitcoin::address::{Address, NetworkChecked};
use bitcoin::{Amount, Block, BlockHash, PublicKey, Txid};
use serde::{Deserialize, Serialize};

use crate::client_sync::{
    into_json, AddNodeCommand, ImportMultiOptions, ImportMultiRequest, ScanAction, ScanObject,
    SetBanCommand,
};
use crate::types::v17::*;

#[rustfmt::skip]                // Keep public re-exports separate.
pub use crate::client_sync::WalletCreateFundedPsbtInput;

crate::define_jsonrpc_minreq_client!("v17");
crate::impl_client_check_expected_server_version!({ [170200] });

// == Blockchain ==
crate::impl_client_v17__getbestblockhash!();
crate::impl_client_v17__getblock!();
crate::impl_client_v17__getblockchaininfo!();
crate::impl_client_v17__getblockcount!();
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
crate::impl_client_v17__verifytxoutproof!();
crate::impl_client_v17__pruneblockchain!();
crate::impl_client_v17__savemempool!();
crate::impl_client_v17__verifychain!();
crate::impl_client_v17__scantxoutset!();

// == Control ==
crate::impl_client_v17__getmemoryinfo!();
crate::impl_client_v17__help!();
crate::impl_client_v17__logging!();
crate::impl_client_v17__stop!();
crate::impl_client_v17__uptime!();

// == Generating ==
crate::impl_client_v17__generatetoaddress!();
crate::impl_client_v17__generate!();
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
crate::impl_client_v17__getpeerinfo!();
crate::impl_client_v17__addnode!();
crate::impl_client_v17__clearbanned!();
crate::impl_client_v17__setban!();
crate::impl_client_v17__listbanned!();
crate::impl_client_v17__disconnectnode!();
crate::impl_client_v17__getconnectioncount!();
crate::impl_client_v17__ping!();
crate::impl_client_v17__setnetworkactive!();

// == Rawtransactions ==
crate::impl_client_v17__createrawtransaction!();
crate::impl_client_v17__fundrawtransaction!();
crate::impl_client_v17__sendrawtransaction!();

// == Wallet ==
crate::impl_client_v17__addmultisigaddress!();
crate::impl_client_v17__bumpfee!();
crate::impl_client_v17__createwallet!();
crate::impl_client_v17__dumpprivkey!();
crate::impl_client_v17__dumpwallet!();
crate::impl_client_v17__getaddressesbylabel!();
crate::impl_client_v17__getaddressinfo!();
crate::impl_client_v17__getbalance!();
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
crate::impl_client_v17__loadwallet!();
crate::impl_client_v17__rescanblockchain!();
crate::impl_client_v17__sendmany!();
crate::impl_client_v17__sendtoaddress!();
crate::impl_client_v17__signmessage!();
crate::impl_client_v17__signrawtransactionwithwallet!();
crate::impl_client_v17__unloadwallet!();
crate::impl_client_v17__walletcreatefundedpsbt!();
crate::impl_client_v17__walletprocesspsbt!();
crate::impl_client_v17__abandontransaction!();
crate::impl_client_v17__abortrescan!();
crate::impl_client_v17__backupwallet!();
crate::impl_client_v17__encryptwallet!();
crate::impl_client_v17__importaddress!();
crate::impl_client_v17__importprivkey!();
crate::impl_client_v17__importprunedfunds!();
crate::impl_client_v17__importpubkey!();
crate::impl_client_v17__importwallet!();
crate::impl_client_v17__keypoolrefill!();
crate::impl_client_v17__lockunspent!();
crate::impl_client_v17__removeprunedfunds!();
crate::impl_client_v17__sethdseed!();
crate::impl_client_v17__settxfee!();
crate::impl_client_v17__walletlock!();
crate::impl_client_v17__walletpassphrase!();
crate::impl_client_v17__walletpassphrasechange!();
crate::impl_client_v17__importmulti!();

/// Argument to the `Client::get_new_address_with_type` function.
///
/// For Core versions 0.17 through to v22. For Core v23 and onwards use `v23::AddressType`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AddressType {
    Legacy,
    P2shSegwit,
    Bech32,
}

impl fmt::Display for AddressType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AddressType::*;

        let s = match *self {
            Legacy => "legacy",
            P2shSegwit => "p2sh-segwit",
            Bech32 => "bech32",
        };
        fmt::Display::fmt(s, f)
    }
}

const SATS_PER_BTC_F64: f64 = 100_000_000.0;

pub fn fee_rate_to_rpc_arg(fee_rate: bitcoin::FeeRate) -> f64 {
    let sat_per_kwu = fee_rate.to_sat_per_kwu();

    let sat_per_kvb = (sat_per_kwu as f64) / 4.0;
    sat_per_kvb / SATS_PER_BTC_F64
}
