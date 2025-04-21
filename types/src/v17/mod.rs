// SPDX-License-Identifier: CC0-1.0

//! # JSON-RPC types for Bitcoin Core `v0.17`.
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
//! | generate                           | done            |
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
//! | sendrawtransaction                 | done            |
//! | signrawtransaction                 | todo            |
//! | signrawtransactionwithkey          | todo            |
//! | testmempoolaccept                  | todo            |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Util == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | createmultisig                     | omitted         |
//! | estimatesmartfee                   | omitted         |
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
//! | getaccount                         | omitted         |
//! | getaccountaddress                  | omitted         |
//! | getaddressbyaccount                | omitted         |
//! | getaddressesbylabel                | done            |
//! | getaddressinfo                     | done (untested) |
//! | getbalance                         | done            |
//! | getnewaddress                      | done            |
//! | getrawchangeaddress                | done            |
//! | getreceivedbyaccount               | omitted         |
//! | getreceivedbyaddress               | done            |
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
//! | listaccounts                       | omitted         |
//! | listaddressgroupings               | done (untested) |
//! | listlabels                         | done (untested) |
//! | listlockunspent                    | done (untested) |
//! | listreceivedbyaccount              | omitted         |
//! | listreceivedbyaddress              | done (untested) |
//! | listsinceblock                     | done (untested) |
//! | listtransactions                   | done (untested) |
//! | listunspent                        | done (untested) |
//! | listwallets                        | done (untested) |
//! | loadwallet                         | done            |
//! | lockunspent                        | omitted         |
//! | move                               | omitted         |
//! | removeprunedfunds                  | omitted         |
//! | rescanblockchain                   | done (untested) |
//! | sendfrom                           | omitted         |
//! | sendmany                           | done (untested) |
//! | sendtoaddress                      | done            |
//! | setaccount                         | omitted         |
//! | sethdseed                          | omitted         |
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

// JSON-RPC types by API section.
mod blockchain;
mod control;
mod generating;
mod mining;
mod network;
mod raw_transactions;
mod util;
mod wallet;
mod zmq;

#[doc(inline)]
pub use self::{
    blockchain::{
        Bip9Softfork, Bip9SoftforkStatus, ChainTips, ChainTipsError, ChainTipsStatus,
        GetBestBlockHash, GetBlockCount, GetBlockHash, GetBlockHeader, GetBlockHeaderError,
        GetBlockHeaderVerbose, GetBlockHeaderVerboseError, GetBlockStats, GetBlockStatsError,
        GetBlockVerboseOne, GetBlockVerboseOneError, GetBlockVerboseZero, GetBlockchainInfo,
        GetBlockchainInfoError, GetChainTips, GetChainTxStats, GetChainTxStatsError, GetDifficulty,
        GetMempoolAncestors, GetMempoolAncestorsVerbose, GetMempoolDescendants,
        GetMempoolDescendantsVerbose, GetMempoolEntry, GetMempoolInfo, GetMempoolInfoError,
        GetRawMempool, GetRawMempoolVerbose, GetTxOut, GetTxOutError, GetTxOutSetInfo,
        GetTxOutSetInfoError, MapMempoolEntryError, MempoolEntry, MempoolEntryError,
        MempoolEntryFees, MempoolEntryFeesError, PruneBlockchain, SaveMempool, ScanTxOutSet,
        ScanTxOutSetUnspent, ScriptPubkey, Softfork, SoftforkReject, VerifyChain, VerifyTxOutProof,
    },
    control::{GetMemoryInfoStats, Locked, Logging},
    generating::{Generate, GenerateToAddress},
    mining::{
        BlockTemplateTransaction, BlockTemplateTransactionError, GetBlockTemplate,
        GetBlockTemplateError, GetMiningInfo,
    },
    network::{
        AddNode, AddedNode, AddedNodeAddress, Banned, ClearBanned, DisconnectNode,
        GetAddedNodeInfo, GetConnectionCount, GetNetTotals, GetNetworkInfo, GetNetworkInfoAddress,
        GetNetworkInfoError, GetNetworkInfoNetwork, GetPeerInfo, ListBanned, PeerInfo, Ping,
        SetBan, SetNetworkActive, UploadTarget,
    },
    raw_transactions::{
        CreateRawTransaction, FundRawTransaction, FundRawTransactionError, SendRawTransaction,
    },
    wallet::{
        AbandonTransaction, AbortRescan, AddMultisigAddress, AddMultisigAddressError,
        AddressInformation, BackupWallet, BumpFee, BumpFeeError, CreateWallet, DumpPrivKey,
        DumpWallet, EncryptWallet, GetAddressInfo, GetAddressInfoEmbedded,
        GetAddressInfoEmbeddedError, GetAddressInfoError, GetAddressInfoLabel, GetAddressesByLabel,
        GetBalance, GetNewAddress, GetRawChangeAddress, GetReceivedByAddress, GetTransaction,
        GetTransactionDetail, GetTransactionDetailError, GetTransactionError,
        GetUnconfirmedBalance, GetWalletInfo, GetWalletInfoError, ImportAddress, ImportMulti,
        ImportPrivKey, ImportPrunedFunds, ImportPubKey, ImportWallet, KeypoolRefill,
        ListAddressGroupings, ListAddressGroupingsError, ListAddressGroupingsItem, ListLabels,
        ListLockUnspent, ListLockUnspentItem, ListLockUnspentItemError, ListReceivedByAddress,
        ListReceivedByAddressError, ListReceivedByAddressItem, ListSinceBlock, ListSinceBlockError,
        ListSinceBlockTransaction, ListSinceBlockTransactionError, ListTransactions,
        ListTransactionsItem, ListTransactionsItemError, ListUnspent, ListUnspentItem,
        ListUnspentItemError, ListWallets, LoadWallet, LockUnspent, RemovePrunedFunds,
        RescanBlockchain, SendMany, SendToAddress, SetHdSeed, SetTxFee, SignErrorData,
        SignErrorDataError, SignMessage, SignRawTransactionWithWallet,
        SignRawTransactionWithWalletError, TransactionCategory, WalletCreateFundedPsbt,
        WalletCreateFundedPsbtError, WalletLock, WalletPassPhrase, WalletPassPhraseChange,
        WalletProcessPsbt,
    },
    zmq::GetZmqNotifications,
};
