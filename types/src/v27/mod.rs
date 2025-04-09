// SPDX-License-Identifier: CC0-1.0

//! # JSON-RPC types for Bitcoin Core `v0.26`
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
//! | dumptxoutset                       | todo            |
//! | getbestblockhash                   | done            |
//! | getblock                           | done            |
//! | getblockchaininfo                  | done            |
//! | getblockcount                      | done            |
//! | getblockfilter                     | done            |
//! | getblockfrompeer                   | todo            |
//! | getblockhash                       | done            |
//! | getblockheader                     | done            |
//! | getblockstats                      | done            |
//! | getchainstates                     | todo            |
//! | getchaintips                       | done            |
//! | getchaintxstats                    | done            |
//! | getdeploymentinfo                  | todo            |
//! | getdifficulty                      | done            |
//! | getmempoolancestors                | done (untested) |
//! | getmempooldescendants              | done (untested) |
//! | getmempoolentry                    | done            |
//! | getmempoolinfo                     | done            |
//! | getrawmempool                      | done            |
//! | gettxout                           | done            |
//! | gettxoutproof                      | done            |
//! | gettxoutsetinfo                    | done            |
//! | gettxspendingprevout               | todo            |
//! | importmempool                      | todo            |
//! | loadtxoutset                       | todo            |
//! | preciousblock                      | done            |
//! | pruneblockchain                    | omitted         |
//! | savemempool                        | omitted         |
//! | scanblocks                         | todo            |
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
//! <summary> Methods from the == Mining == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | getblocktemplate                   | done            |
//! | getmininginfo                      | done            |
//! | getnetworkhashps                   | done            |
//! | getprioritisedtransactions         | done            |
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
//! | getaddrmaninfo                     | todo            |
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
//! | combinepsbt                        | done            |
//! | combinerawtransaction              | done            |
//! | converttopsbt                      | done            |
//! | createpsbt                         | done            |
//! | createrawtransaction               | done            |
//! | decodepsbt                         | done            |
//! | decoderawtransaction               | done            |
//! | decodescript                       | done            |
//! | descriptorprocesspsbt              | todo            |
//! | finalizepsbt                       | done (untested) |
//! | fundrawtransaction                 | done (untested) |
//! | getrawtransaction                  | done            |
//! | joinpsbts                          | todo            |
//! | sendrawtransaction                 | done            |
//! | signrawtransactionwithkey          | done (untested) |
//! | submitpackage                      | done            |
//! | testmempoolaccept                  | done (untested) |
//! | utxoupdatepsbt                     | todo            |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Signer == section </summary>
//!
//! | JSON-PRC Method Name               | Status          |
//! |:-----------------------------------|:---------------:|
//! | enumeratesigners                   | todo            |
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
//! | getindexinfo                       | todo            |
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
//! | importdescriptors                  | todo            |
//! | importmulti                        | omitted         |
//! | importprivkey                      | omitted         |
//! | importprunedfunds                  | omitted         |
//! | importpubkey                       | omitted         |
//! | importwallet                       | omitted         |
//! | keypoolrefill                      | omitted         |
//! | listaddressgroupings               | done (untested) |
//! | listdescriptors                    | todo            |
//! | listlabels                         | done (untested) |
//! | listlockunspent                    | done (untested) |
//! | migratewallet                      | todo            |
//! | newkeypool                         | todo            |
//! | psbtbumpfee                        | todo            |
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
//! | restorewallet                      | todo            |
//! | send                               | todo            |
//! | sendall                            | todo            |
//! | sendmany                           | done (untested) |
//! | sendtoaddress                      | done            |
//! | sethdseed                          | omitted         |
//! | setlabel                           | todo            |
//! | settxfee                           | omitted         |
//! | setwalletflag                      | todo            |
//! | signmessage                        | done (untested) |
//! | signrawtransactionwithwallet       | done (untested) |
//! | simulaterawtransaction             | todo            |
//! | unloadwallet                       | done            |
//! | upgradewallet                      | todo            |
//! | walletcreatefundedpsbt             | done (untested) |
//! | walletdisplayaddress               | todo            |
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

#[doc(inline)]
pub use crate::{
    v17::{
        AddMultisigAddress, AddedNode, AddedNodeAddress, AddressInformation, Banned, BumpFee,
        ChainTips, ChainTipsStatus, CombinePsbt, CombineRawTransaction, ConvertToPsbt, CreatePsbt,
        CreateRawTransaction, DecodePsbt, DecodeRawTransaction, DecodeScript, DecodeScriptError,
        DumpPrivKey, DumpWallet, FinalizePsbt, FundRawTransaction, FundRawTransactionError,
        Generate, GenerateToAddress, GetAddedNodeInfo, GetAddressInfo, GetAddressInfoEmbedded,
        GetAddressInfoLabel, GetAddressesByLabel, GetBalance, GetBestBlockHash, GetBlockCount,
        GetBlockHash, GetBlockHeader, GetBlockHeaderVerbose, GetBlockStats, GetBlockTemplate,
        GetBlockTemplateError, GetBlockVerbosityOne, GetBlockVerbosityZero, GetChainTips,
        GetChainTxStats, GetDifficulty, GetMemoryInfoStats, GetMempoolInfo, GetMiningInfo,
        GetNetTotals, GetNetworkInfo, GetNetworkInfoAddress, GetNetworkInfoError,
        GetNetworkInfoNetwork, GetNewAddress, GetPeerInfo, GetRawChangeAddress, GetRawMempool,
        GetRawMempoolVerbose, GetRawTransaction, GetRawTransactionVerbose,
        GetRawTransactionVerboseError, GetReceivedByAddress, GetTransaction, GetTransactionDetail,
        GetUnconfirmedBalance, GetWalletInfo, GetZmqNotifications, ListAddressGroupings,
        ListAddressGroupingsItem, ListBanned, ListLabels, ListLockUnspent, ListLockUnspentItem,
        ListReceivedByAddress, ListReceivedByAddressItem, ListSinceBlock,
        ListSinceBlockTransaction, ListTransactions, ListTransactionsItem, ListUnspent,
        ListUnspentItem, ListWallets, Locked, PeerInfo, RawTransactionError, RawTransactionInput,
        RawTransactionOutput, RescanBlockchain, SendMany, SendRawTransaction, SendToAddress,
        SignMessage, SignRawTransaction, SignRawTransactionError, SoftforkReject,
        TestMempoolAccept, TransactionCategory, UploadTarget, VerifyTxOutProof,
        WalletCreateFundedPsbt, WalletProcessPsbt, WitnessUtxo,
    },
    v18::{ActiveCommand, GetRpcInfo},
    v19::{
        Bip9SoftforkInfo, Bip9SoftforkStatistics, Bip9SoftforkStatus, GetBalances, GetBalancesMine,
        GetBalancesWatchOnly, GetBlockFilter, GetBlockFilterError, GetBlockchainInfo,
        GetBlockchainInfoError, GetMempoolAncestors, GetMempoolAncestorsVerbose,
        GetMempoolDescendants, GetMempoolDescendantsVerbose, GetMempoolEntry, MapMempoolEntryError,
        MempoolEntry, MempoolEntryError, MempoolEntryFees, MempoolEntryFeesError, Softfork,
        SoftforkType,
    },
    v22::{GetTxOut, GetTxOutError, Logging, ScriptPubkey},
    v26::{
        CreateWallet, GetPrioritisedTransactions, GetTxOutSetInfo, GetTxOutSetInfoError,
        LoadWallet, PrioritisedTransaction, SubmitPackage, SubmitPackageError,
        SubmitPackageTxResult, SubmitPackageTxResultError, SubmitPackageTxResultFees,
        SubmitPackageTxResultFeesError, UnloadWallet,
    },
};
