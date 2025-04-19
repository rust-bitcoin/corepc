// SPDX-License-Identifier: CC0-1.0

// TODO: Work out how to solve the problem that the docs on the re-exported types are for v17.
//
// We probably need to write a script to pull the v18 docs from Core (code base or RPC call) and
// check them against the v17 docs for differences.

//! # JSON-RPC types for Bitcoin Core `v0.18`
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
//! | getblockhash                       | done            |
//! | getblockheader                     | done            |
//! | getblockstats                      | done            |
//! | getchaintips                       | done            |
//! | getchaintxstats                    | done            |
//! | getdifficulty                      | done            |
//! | getmempoolancestors                | done (untested) |
//! | getmempooldescendants              | done (untested) |
//! | getmempoolentry                    | done            |
//! | getmempoolinfo                     | done            |
//! | getrawmempool                      | done            |
//! | gettxout                           | done            |
//! | gettxoutproof                      | done            |
//! | gettxoutsetinfo                    | done            |
//! | preciousblock                      | done            |
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
//! | getrpcinfo                         | done            |
//! | help                               | done            |
//! | logging                            | done            |
//! | stop                               | done            |
//! | uptime                             | done            |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Generating == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | generate                           | omitted         |
//! | generatetoaddress                  | done            |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Mining == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | getblocktemplate                   | done            |
//! | getmininginfo                      | done            |
//! | getnetworkhashps                   | done            |
//! | prioritisetransaction              | done            |
//! | submitblock                        | done (untested) |
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
//! | createrawtransaction               | done            |
//! | decodepsbt                         | todo            |
//! | decoderawtransaction               | todo            |
//! | decodescript                       | todo            |
//! | finalizepsbt                       | todo            |
//! | fundrawtransaction                 | done (untested) |
//! | getrawtransaction                  | todo            |
//! | joinpsbts                          | todo            |
//! | sendrawtransaction                 | done            |
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
//! | signmessage                        | done (untested) |
//! | signrawtransactionwithwallet       | done (untested) |
//! | unloadwallet                       | done            |
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

// JSON-RPC types by API section.
mod control;

#[doc(inline)]
pub use self::control::{ActiveCommand, GetRpcInfo};
#[doc(inline)]
pub use crate::v17::{
    AddMultisigAddress, AddMultisigAddressError, AddedNode, AddedNodeAddress, AddressInformation,
    Banned, Bip9Softfork, Bip9SoftforkStatus, BumpFee, BumpFeeError, ChainTips, ChainTipsError,
    ChainTipsStatus, CreateRawTransaction, CreateWallet, DumpPrivKey, DumpWallet,
    FundRawTransaction, FundRawTransactionError, Generate, GenerateToAddress, GetAddedNodeInfo,
    GetAddressInfo, GetAddressInfoEmbedded, GetAddressInfoEmbeddedError, GetAddressInfoError,
    GetAddressInfoLabel, GetAddressesByLabel, GetBalance, GetBestBlockHash, GetBlockCount,
    GetBlockHash, GetBlockHeader, GetBlockHeaderError, GetBlockHeaderVerbose,
    GetBlockHeaderVerboseError, GetBlockStats, GetBlockStatsError, GetBlockTemplate,
    GetBlockTemplateError, GetBlockVerboseOne, GetBlockVerboseOneError, GetBlockVerboseZero,
    GetBlockchainInfo, GetBlockchainInfoError, GetChainTips, GetChainTxStats, GetChainTxStatsError,
    GetDifficulty, GetMemoryInfoStats, GetMempoolAncestors, GetMempoolAncestorsVerbose,
    GetMempoolDescendants, GetMempoolDescendantsVerbose, GetMempoolEntry, GetMempoolInfo,
    GetMempoolInfoError, GetMiningInfo, GetNetTotals, GetNetworkInfo, GetNetworkInfoAddress,
    GetNetworkInfoError, GetNetworkInfoNetwork, GetNewAddress, GetPeerInfo, GetRawChangeAddress,
    GetRawMempool, GetRawMempoolVerbose, GetReceivedByAddress, GetTransaction,
    GetTransactionDetail, GetTransactionDetailError, GetTransactionError, GetTxOut, GetTxOutError,
    GetTxOutSetInfo, GetTxOutSetInfoError, GetUnconfirmedBalance, GetWalletInfo,
    GetWalletInfoError, GetZmqNotifications, ListAddressGroupings, ListAddressGroupingsError,
    ListAddressGroupingsItem, ListBanned, ListLabels, ListLockUnspent, ListLockUnspentItem,
    ListLockUnspentItemError, ListReceivedByAddress, ListReceivedByAddressError,
    ListReceivedByAddressItem, ListSinceBlock, ListSinceBlockError, ListSinceBlockTransaction,
    ListSinceBlockTransactionError, ListTransactions, ListTransactionsItem,
    ListTransactionsItemError, ListUnspent, ListUnspentItem, ListUnspentItemError, ListWallets,
    LoadWallet, Locked, Logging, MapMempoolEntryError, MempoolEntry, MempoolEntryError,
    MempoolEntryFees, MempoolEntryFeesError, PeerInfo, RescanBlockchain, ScriptPubkey, SendMany,
    SendRawTransaction, SendToAddress, SignErrorData, SignErrorDataError, SignMessage,
    SignRawTransactionWithWallet, SignRawTransactionWithWalletError, Softfork, SoftforkReject,
    TransactionCategory, UploadTarget, VerifyTxOutProof, WalletCreateFundedPsbt,
    WalletCreateFundedPsbtError, WalletProcessPsbt, SetNetworkActive, SaveMempool, VerifyChain, AbandonTransaction, AbortRescan, BackupWallet, EncryptWallet, ImportAddress, ImportPrivKey, ImportPrunedFunds, ImportPubKey, ImportWallet, KeypoolRefill, LockUnspent, RemovePrunedFunds, SetHdSeed, SetTxFee, WalletLock, WalletPassPhrase, WalletPassPhraseChange,
};
