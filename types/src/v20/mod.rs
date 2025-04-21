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
//! | getblockfilter                     | done            |
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
mod network;
mod wallet;

#[doc(inline)]
pub use self::control::Logging;
#[doc(inline)]
pub use crate::{
    v17::{
        AbandonTransaction, AddMultisigAddress, AddMultisigAddressError, AddedNode,
        AddedNodeAddress, AddressInformation, BackupWallet, Banned, BumpFee, BumpFeeError,
        ChainTips, ChainTipsError, ChainTipsStatus, CreateRawTransaction, CreateWallet,
        DumpPrivKey, DumpWallet, FundRawTransaction, FundRawTransactionError, Generate,
        GenerateToAddress, GetAddedNodeInfo, GetAddressInfo, GetAddressInfoEmbedded,
        GetAddressInfoError, GetAddressInfoLabel, GetAddressesByLabel, GetBalance,
        GetBestBlockHash, GetBlockCount, GetBlockHash, GetBlockHeader, GetBlockHeaderError,
        GetBlockHeaderVerbose, GetBlockHeaderVerboseError, GetBlockStats, GetBlockStatsError,
        GetBlockTemplate, GetBlockTemplateError, GetBlockVerboseOne, GetBlockVerboseOneError,
        GetBlockVerboseZero, GetChainTips, GetChainTxStats, GetChainTxStatsError, GetDifficulty,
        GetMemoryInfoStats, GetMempoolInfo, GetMempoolInfoError, GetMiningInfo, GetNetTotals,
        GetNetworkInfo, GetNetworkInfoAddress, GetNetworkInfoError, GetNetworkInfoNetwork,
        GetNewAddress, GetPeerInfo, GetRawChangeAddress, GetRawMempool, GetRawMempoolVerbose,
        GetReceivedByAddress, GetTransaction, GetTransactionDetail, GetTransactionError, GetTxOut,
        GetTxOutError, GetTxOutSetInfo, GetTxOutSetInfoError, GetUnconfirmedBalance, GetWalletInfo,
        GetZmqNotifications, ImportAddress, ImportPrivKey, ImportPrunedFunds, ImportPubKey,
        ImportWallet, KeypoolRefill, ListAddressGroupings, ListAddressGroupingsItem, ListBanned,
        ListLabels, ListLockUnspent, ListLockUnspentItem, ListReceivedByAddress,
        ListReceivedByAddressItem, ListSinceBlock, ListSinceBlockTransaction, ListTransactions,
        ListTransactionsItem, ListUnspent, ListUnspentItem, ListWallets, LoadWallet, LockUnspent,
        Locked, PeerInfo, RemovePrunedFunds, RescanBlockchain, SaveMempool, ScriptPubkey, SendMany,
        SendRawTransaction, SendToAddress, SetHdSeed, SetTxFee, SignErrorData, SignMessage,
        SignRawTransactionWithWallet, SoftforkReject, TransactionCategory, UploadTarget,
        VerifyChain, VerifyTxOutProof, WalletCreateFundedPsbt, WalletLock, WalletPassPhrase,
        WalletPassPhraseChange, WalletProcessPsbt,
    },
    v18::{ActiveCommand, GetRpcInfo},
    v19::{
        Bip9SoftforkInfo, Bip9SoftforkStatistics, Bip9SoftforkStatus, GetBalances, GetBalancesMine,
        GetBalancesWatchOnly, GetBlockFilter, GetBlockFilterError, GetBlockchainInfo,
        GetBlockchainInfoError, GetMempoolAncestors, GetMempoolAncestorsVerbose,
        GetMempoolDescendants, GetMempoolDescendantsVerbose, GetMempoolEntry, MapMempoolEntryError,
        MempoolEntry, MempoolEntryError, MempoolEntryFees, MempoolEntryFeesError, ScanTxOutSet,
        Softfork, SoftforkType,
    },
    v20::{
        network::SetNetworkActive,
        wallet::{AbortRescan, EncryptWallet},
    },
};
