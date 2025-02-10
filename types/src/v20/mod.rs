// SPDX-License-Identifier: CC0-1.0

//! # JSON-RPC types for Bitcoin Core `v0.20`
//!
//! These structs are shaped for the JSON data returned by the JSON-RPC API. They use stdlib types
//! (or custom types) and where necessary implement an `into_model` function to convert the type to
//! a [`crate::model`] type of the same name. The types in this module are version specific. The
//! types in the `model` module are version nonspecific and are strongly typed using `rust-bitcoin`.
//!
//! ### Method name and implementation status
//!
//! Every JSON-RPC method supported by this version of Bitcoin Core is listed below along with its
//! current implementation status.
//!
//! <details>
//! <summary> Methods from the == Blockchain == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | getbestblockhash                   | done            |
//! | getblock                           | done            |
//! | getblockchaininfo                  | done            |
//! | getblockcount                      | done            |
//! | getblockfilter                     | todo            |
//! | getblockhash                       | done            |
//! | getblockheader                     | done            |
//! | getblockstats                      | done (untested) |
//! | getchaintips                       | done            |
//! | getchaintxstats                    | done            |
//! | getdifficulty                      | done            |
//! | getmempoolancestors                | done (untested) |
//! | getmempooldescendants              | done (untested) |
//! | getmempoolentry                    | done            |
//! | getmempoolinfo                     | done            |
//! | getrawmempool                      | done            |
//! | gettxout                           | done            |
//! | gettxoutproof                      | omitted         |
//! | gettxoutsetinfo                    | done            |
//! | preciousblock                      | omitted         |
//! | pruneblockchain                    | omitted         |
//! | savemempool                        | omitted         |
//! | scantxoutset                       | omitted         |
//! | verifychain                        | omitted         |
//! | verifytxoutproof                   | done            |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Control == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | getmemoryinfo                      | done            |
//! | getrpcinfo                         | todo            |
//! | help                               | omitted         |
//! | logging                            | done            |
//! | stop                               | omitted         |
//! | uptime                             | omitted         |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Generating == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | generatetoaddress                  | done            |
//! | generatetodescriptor               | todo            |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Mining == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | getblocktemplate                   | todo            |
//! | getmininginfo                      | todo            |
//! | getnetworkhashps                   | todo            |
//! | prioritisetransaction              | todo            |
//! | submitblock                        | todo            |
//! | submitheader                       | todo            |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Network == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | addnode                            | omitted         |
//! | clearbanned                        | omitted         |
//! | disconnectnode                     | omitted         |
//! | getaddednodeinfo                   | done            |
//! | getconnectioncount                 | omitted         |
//! | getnettotals                       | done            |
//! | getnetworkinfo                     | done            |
//! | getnodeaddresses                   | todo            |
//! | getpeerinfo                        | done            |
//! | listbanned                         | omitted         |
//! | ping                               | omitted         |
//! | setban                             | omitted         |
//! | setnetworkactive                   | omitted         |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Rawtransactions == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | analyzepsbt                        | todo            |
//! | combinepsbt                        | todo            |
//! | combinerawtransaction              | todo            |
//! | converttopsbt                      | todo            |
//! | createpsbt                         | todo            |
//! | createrawtransaction               | todo            |
//! | decodepsbt                         | todo            |
//! | decoderawtransaction               | todo            |
//! | decodescript                       | todo            |
//! | finalizepsbt                       | todo            |
//! | fundrawtransaction                 | todo            |
//! | getrawtransaction                  | todo            |
//! | joinpsbts                          | todo            |
//! | sendrawtransaction                 | done (untested) |
//! | signrawtransactionwithkey          | todo            |
//! | testmempoolaccept                  | todo            |
//! | utxoupdatepsbt                     | todo            |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Util == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | createmultisig                     | omitted         |
//! | deriveaddresses                    | todo            |
//! | estimatesmartfee                   | omitted         |
//! | getdescriptorinfo                  | todo            |
//! | signmessagewithprivkey             | omitted         |
//! | validateaddress                    | omitted         |
//! | verifymessage                      | omitted         |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Wallet == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | abandontransaction                 | omitted         |
//! | abortrescan                        | omitted         |
//! | addmultisigaddress                 | done (untested) |
//! | backupwallet                       | omitted         |
//! | bumpfee                            | done            |
//! | createwallet                       | done            |
//! | dumpprivkey                        | done            |
//! | dumpwallet                         | done            |
//! | encryptwallet                      | omitted         |
//! | getaddressesbylabel                | done            |
//! | getaddressinfo                     | done (untested) |
//! | getbalance                         | done            |
//! | getbalances                        | done            |
//! | getnewaddress                      | done            |
//! | getrawchangeaddress                | done            |
//! | getreceivedbyaddress               | done            |
//! | getreceivedbylabel                 | todo            |
//! | gettransaction                     | done            |
//! | getunconfirmedbalance              | done (untested) |
//! | getwalletinfo                      | done (untested) |
//! | importaddress                      | omitted         |
//! | importmulti                        | omitted         |
//! | importprivkey                      | omitted         |
//! | importprunedfunds                  | omitted         |
//! | importpubkey                       | omitted         |
//! | importwallet                       | omitted         |
//! | keypoolrefill                      | omitted         |
//! | listaddressgroupings               | done (untested) |
//! | listlabels                         | done (untested) |
//! | listlockunspent                    | done (untested) |
//! | listreceivedbyaddress              | done (untested) |
//! | listreceivedbylabel                | todo            |
//! | listsinceblock                     | done (untested) |
//! | listtransactions                   | done (untested) |
//! | listunspent                        | done (untested) |
//! | listwalletdir                      | todo            |
//! | listwallets                        | done (untested) |
//! | loadwallet                         | done            |
//! | lockunspent                        | omitted         |
//! | removeprunedfunds                  | omitted         |
//! | rescanblockchain                   | done (untested) |
//! | sendmany                           | done (untested) |
//! | sendtoaddress                      | done            |
//! | sethdseed                          | omitted         |
//! | setlabel                           | todo            |
//! | settxfee                           | omitted         |
//! | setwalletflag                      | todo            |
//! | signmessage                        | done (untested) |
//! | signrawtransactionwithwallet       | done (untested) |
//! | unloadwallet                       | omitted         |
//! | walletcreatefundedpsbt             | done (untested) |
//! | walletlock                         | omitted         |
//! | walletpassphrase                   | omitted         |
//! | walletpassphrasechange             | omitted         |
//! | walletprocesspsbt                  | done (untested) |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Zmq == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | getzmqnotifications                | done (untested) |
//!
//! </details>
//!
//!
//! **Items marked omitted were omitted because:**
//!
//! - Method does not return anything.
//! - Method returns a simple type (e.g. bool or integer).
//! - Method is deprecated.

#[doc(inline)]
pub use crate::{
    v17::{
        AddMultisigAddress, AddedNode, AddedNodeAddress, AddressInformation, Banned, BumpFee,
        ChainTips, ChainTipsStatus, CreateWallet, DumpPrivKey, DumpWallet, Generate,
        GenerateToAddress, GetAddedNodeInfo, GetAddressInfo, GetAddressInfoEmbedded,
        GetAddressInfoLabel, GetAddressesByLabel, GetBalance, GetBestBlockHash, GetBlockCount,
        GetBlockHash, GetBlockHeader, GetBlockHeaderVerbose, GetBlockStats, GetBlockVerbosityOne,
        GetBlockVerbosityZero, GetChainTips, GetChainTxStats, GetDifficulty, GetMemoryInfoStats,
        GetMempoolAncestors, GetMempoolAncestorsVerbose, GetMempoolDescendants,
        GetMempoolDescendantsVerbose, GetMempoolEntry, GetMempoolInfo, GetNetTotals,
        GetNetworkInfo, GetNetworkInfoAddress, GetNetworkInfoError, GetNetworkInfoNetwork,
        GetNewAddress, GetPeerInfo, GetRawChangeAddress, GetRawMempool, GetRawMempoolVerbose,
        GetReceivedByAddress, GetTransaction, GetTransactionDetail, GetTxOut, GetTxOutSetInfo,
        GetUnconfirmedBalance, GetWalletInfo, GetZmqNotifications, ListAddressGroupings,
        ListAddressGroupingsItem, ListBanned, ListLabels, ListLockUnspent, ListLockUnspentItem,
        ListReceivedByAddress, ListReceivedByAddressItem, ListSinceBlock,
        ListSinceBlockTransaction, ListTransactions, ListTransactionsItem, ListUnspent,
        ListUnspentItem, ListWallets, LoadWallet, Locked, Logging, MempoolEntry, MempoolEntryFees,
        PeerInfo, RescanBlockchain, ScriptPubkey, SendMany, SendRawTransaction, SendToAddress,
        SignErrorData, SignMessage, SignRawTransactionWithWallet, SoftforkReject,
        TransactionCategory, UploadTarget, Uptime, VerifyTxOutProof, WalletCreateFundedPsbt,
        WalletProcessPsbt,
    },
    v19::{
        Bip9SoftforkInfo, Bip9SoftforkStatistics, Bip9SoftforkStatus, GetBalances, GetBalancesMine,
        GetBalancesWatchOnly, GetBlockchainInfo, Softfork, SoftforkType,
    },
};
