// SPDX-License-Identifier: CC0-1.0

//! # JSON-RPC types for Bitcoin Core `v23`
//!
//! These structs are shaped for the JSON data returned by the JSON-RPC API. They use stdlib types
//! (or custom types) and where necessary implement an `into_model` function to convert the type to
//! a [`crate::model`] type of the same name. The types in this module are version specific. The
//! types in the `model` module are version nonspecific and are strongly typed using `rust-bitcoin`.
//!
//! ### Method name and implementation status
//!
//! Every JSON-RPC method supported by this version of Bitcoin Core is listed below along with the
//! type it returns and any implementation notes.
//!
//! Key to 'Returns' column:
//!
//! * version: method returns a version specific type but has no model type.
//! * version + model: method returns a version specific type and can be converted to a model type.
//! * returns foo: method returns a foo (e.g. string, boolean, or nothing).
//! * omitted: method intentionally unsupported with no plans of adding support.
//!
//! If a method has UNTESTED then there is no integration test yet for it.
//!
//! <details>
//! <summary> Methods from the == Blockchain == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | getbestblockhash                   | version + model |                                        |
//! | getblock                           | version + model | Includes additional 'verbose' type     |
//! | getblockchaininfo                  | version + model |                                        |
//! | getblockcount                      | version + model |                                        |
//! | getblockfilter                     | version         |                                        |
//! | getblockfrompeer                   | version + model | TODO                                   |
//! | getblockhash                       | version + model |                                        |
//! | getblockheader                     | version + model | Includes additional 'verbose' type     |
//! | getblockstats                      | version + model |                                        |
//! | getchaintips                       | version + model |                                        |
//! | getchaintxstats                    | version + model |                                        |
//! | getdeploymentinfo                  | version + model | TODO                                   |
//! | getdifficulty                      | version + model |                                        |
//! | getmempoolancestors                | version + model | UNTESTED (incl. verbose type)          |
//! | getmempooldescendants              | version + model | UNTESTED (incl. verbose type)          |
//! | getmempoolentry                    | version + model |                                        |
//! | getmempoolinfo                     | version + model |                                        |
//! | getrawmempool                      | version + model | Includes additional 'verbose' type     |
//! | gettxout                           | version + model |                                        |
//! | gettxoutproof                      | returns string  |                                        |
//! | gettxoutsetinfo                    | version + model |                                        |
//! | preciousblock                      | returns nothing |                                        |
//! | pruneblockchain                    | version         |                                        |
//! | savemempool                        | version         |                                        |
//! | scantxoutset                       | version + model | API marked as experimental             |
//! | verifychain                        | version         |                                        |
//! | verifytxoutproof                   | version + model |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Control == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | getmemoryinfo                      | version         |                                        |
//! | getrpcinfo                         | version + model |                                        |
//! | help                               | returns string  |                                        |
//! | logging                            | version         |                                        |
//! | stop                               | returns string  |                                        |
//! | uptime                             | returns numeric |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Mining == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | getblocktemplate                   | version + model |                                        |
//! | getmininginfo                      | version + model |                                        |
//! | getnetworkhashps                   | returns boolean |                                        |
//! | prioritisetransaction              | returns boolean |                                        |
//! | submitblock                        | returns nothing |                                        |
//! | submitheader                       | returns nothing |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Network == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | addnode                            | returns nothing |                                        |
//! | clearbanned                        | returns nothing |                                        |
//! | disconnectnode                     | returns nothing |                                        |
//! | getaddednodeinfo                   | version         |                                        |
//! | getconnectioncount                 | version         |                                        |
//! | getnettotals                       | version         |                                        |
//! | getnetworkinfo                     | version + model |                                        |
//! | getnodeaddresses                   | version + model |                                        |
//! | getpeerinfo                        | version         |                                        |
//! | listbanned                         | version         |                                        |
//! | ping                               | returns nothing |                                        |
//! | setban                             | returns nothing |                                        |
//! | setnetworkactive                   | version         |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Rawtransactions == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | analyzepsbt                        | version + model |                                        |
//! | combinepsbt                        | version + model |                                        |
//! | combinerawtransaction              | version + model |                                        |
//! | converttopsbt                      | version + model |                                        |
//! | createpsbt                         | version + model |                                        |
//! | createrawtransaction               | version + model |                                        |
//! | decodepsbt                         | version + model |                                        |
//! | decoderawtransaction               | version + model |                                        |
//! | decodescript                       | version + model |                                        |
//! | finalizepsbt                       | version + model | UNTESTED                               |
//! | fundrawtransaction                 | version + model |                                        |
//! | getrawtransaction                  | version + model | Includes additional 'verbose' type     |
//! | joinpsbts                          | version + model | UNTESTED                               |
//! | sendrawtransaction                 | version + model |                                        |
//! | signrawtransactionwithkey          | version + model |                                        |
//! | testmempoolaccept                  | version + model | UNTESTED                               |
//! | utxoupdatepsbt                     | version + model | UNTESTED                               |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Signer == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | enumeratesigners                   | version         | UNTESTED                               |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Util == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | createmultisig                     | version + model |                                        |
//! | deriveaddresses                    | version + model |                                        |
//! | estimatesmartfee                   | version + model |                                        |
//! | getdescriptorinfo                  | version         |                                        |
//! | getindexinfo                       | version         |                                        |
//! | signmessagewithprivkey             | version + model |                                        |
//! | validateaddress                    | version + model |                                        |
//! | verifymessage                      | version         |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Wallet == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | abandontransaction                 | returns nothing |                                        |
//! | abortrescan                        | version         |                                        |
//! | addmultisigaddress                 | version + model | UNTESTED                               |
//! | backupwallet                       | returns nothing |                                        |
//! | bumpfee                            | version + model |                                        |
//! | createwallet                       | version + model |                                        |
//! | dumpprivkey                        | version + model |                                        |
//! | dumpwallet                         | version + model |                                        |
//! | encryptwallet                      | version         |                                        |
//! | getaddressesbylabel                | version + model |                                        |
//! | getaddressinfo                     | version + model | UNTESTED                               |
//! | getbalance                         | version + model |                                        |
//! | getbalances                        | version + model |                                        |
//! | getnewaddress                      | version + model |                                        |
//! | getrawchangeaddress                | version + model |                                        |
//! | getreceivedbyaddress               | version + model |                                        |
//! | getreceivedbylabel                 | version + model |                                        |
//! | gettransaction                     | version + model |                                        |
//! | getunconfirmedbalance              | version + model | UNTESTED                               |
//! | getwalletinfo                      | version + model | UNTESTED                               |
//! | importaddress                      | returns nothing |                                        |
//! | importdescriptors                  | version         |                                        |
//! | importmulti                        | version         |                                        |
//! | importprivkey                      | returns nothing |                                        |
//! | importprunedfunds                  | returns nothing |                                        |
//! | importpubkey                       | returns nothing |                                        |
//! | importwallet                       | returns nothing |                                        |
//! | keypoolrefill                      | returns nothing |                                        |
//! | listaddressgroupings               | version + model | UNTESTED                               |
//! | listdescriptors                    | version         |                                        |
//! | listlabels                         | version + model | UNTESTED                               |
//! | listlockunspent                    | version + model | UNTESTED                               |
//! | newkeypool                         | version + model | TODO                                   |
//! | psbtbumpfee                        | version + model |                                        |
//! | listreceivedbyaddress              | version + model | UNTESTED                               |
//! | listreceivedbylabel                | version + model |                                        |
//! | listsinceblock                     | version + model | UNTESTED                               |
//! | listtransactions                   | version + model | UNTESTED                               |
//! | listunspent                        | version + model |                                        |
//! | listwalletdir                      | version         |                                        |
//! | listwallets                        | version + model | UNTESTED                               |
//! | loadwallet                         | version + model |                                        |
//! | lockunspent                        | version         |                                        |
//! | removeprunedfunds                  | returns nothing |                                        |
//! | rescanblockchain                   | version + model | UNTESTED                               |
//! | restorewallet                      | version + model | TODO                                   |
//! | send                               | version + model |                                        |
//! | sendmany                           | version + model | UNTESTED                               |
//! | sendtoaddress                      | version + model |                                        |
//! | sethdseed                          | returns nothing |                                        |
//! | setlabel                           | returns nothing |                                        |
//! | settxfee                           | version         |                                        |
//! | setwalletflag                      | version         |                                        |
//! | signmessage                        | version + model |                                        |
//! | signrawtransactionwithwallet       | version + model |                                        |
//! | unloadwallet                       | returns nothing |                                        |
//! | upgradewallet                      | version         |                                        |
//! | walletcreatefundedpsbt             | version + model | UNTESTED                               |
//! | walletdisplayaddress               | version + model | UNTESTED                               |
//! | walletlock                         | returns nothing |                                        |
//! | walletpassphrase                   | returns nothing |                                        |
//! | walletpassphrasechange             | returns nothing |                                        |
//! | walletprocesspsbt                  | version + model | UNTESTED                               |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Zmq == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | getzmqnotifications                | version         | UNTESTED                               |
//!
//! </details>

// JSON-RPC types by API section.
mod blockchain;
mod control;
mod network;
mod raw_transactions;
mod util;
mod wallet;

#[doc(inline)]
pub use self::{
    blockchain::{GetBlockchainInfo, GetMempoolEntry, SaveMempool},
    control::Logging,
    network::GetPeerInfo,
    raw_transactions::{
        DecodePsbt, DecodePsbtError, DecodeScript, DecodeScriptError, GlobalXpub, Proprietary,
        PsbtInput, PsbtOutput,
    },
    util::CreateMultisig,
    wallet::{GetTransaction, GetTransactionError},
};
#[doc(inline)]
pub use crate::{
    v17::{
        AbortRescan, AddMultisigAddress, AddMultisigAddressError, AddedNode, AddedNodeAddress,
        AddressInformation, Bip125Replaceable, BumpFee, BumpFeeError, ChainTips, ChainTipsError,
        ChainTipsStatus, CombinePsbt, CombineRawTransaction, ConvertToPsbt, CreateMultisigError,
        CreatePsbt, CreateRawTransaction, CreateWallet, DecodeRawTransaction, DumpPrivKey,
        DumpWallet, EncryptWallet, EstimateSmartFee, FinalizePsbt, FinalizePsbtError,
        FundRawTransaction, FundRawTransactionError, Generate, GenerateToAddress, GetAddedNodeInfo,
        GetAddressInfo, GetAddressInfoEmbedded, GetAddressInfoEmbeddedError, GetAddressInfoError,
        GetAddressInfoLabel, GetAddressesByLabel, GetBalance, GetBestBlockHash, GetBlockCount,
        GetBlockHash, GetBlockHeader, GetBlockHeaderError, GetBlockHeaderVerbose,
        GetBlockHeaderVerboseError, GetBlockStats, GetBlockStatsError, GetBlockTemplate,
        GetBlockTemplateError, GetBlockVerboseOne, GetBlockVerboseOneError, GetBlockVerboseZero,
        GetChainTips, GetChainTxStatsError, GetConnectionCount, GetDifficulty, GetMemoryInfoStats,
        GetMempoolInfoError, GetMiningInfo, GetNetTotals, GetNetworkInfoAddress,
        GetNetworkInfoError, GetNetworkInfoNetwork, GetNewAddress, GetRawChangeAddress,
        GetRawMempool, GetRawMempoolVerbose, GetRawTransaction, GetRawTransactionVerbose,
        GetRawTransactionVerboseError, GetReceivedByAddress, GetTransactionDetailError, GetTxOut,
        GetTxOutError, GetTxOutSetInfo, GetTxOutSetInfoError, GetUnconfirmedBalance, GetWalletInfo,
        GetWalletInfoError, GetZmqNotifications, ListAddressGroupings, ListAddressGroupingsError,
        ListAddressGroupingsItem, ListLabels, ListLockUnspent, ListLockUnspentItem,
        ListLockUnspentItemError, ListReceivedByAddress, ListReceivedByAddressError,
        ListReceivedByAddressItem, ListSinceBlock, ListSinceBlockError, ListSinceBlockTransaction,
        ListSinceBlockTransactionError, ListTransactions, ListTransactionsItem,
        ListTransactionsItemError, ListUnspentItemError, ListWallets, LoadWallet, LockUnspent,
        Locked, PruneBlockchain, RawTransactionError, RawTransactionInput, RawTransactionOutput,
        RescanBlockchain, ScanTxOutSetAbort, ScanTxOutSetError, ScanTxOutSetStatus, SendMany,
        SendRawTransaction, SendToAddress, SetNetworkActive, SetTxFee, SignMessage,
        SignMessageWithPrivKey, SignRawTransaction, SignRawTransactionError, SoftforkReject,
        TestMempoolAccept, TransactionCategory, UploadTarget, ValidateAddress,
        ValidateAddressError, VerifyChain, VerifyMessage, VerifyTxOutProof, WalletCreateFundedPsbt,
        WalletCreateFundedPsbtError, WalletProcessPsbt, WitnessUtxo,
    },
    v18::{
        ActiveCommand, AnalyzePsbt, AnalyzePsbtError, AnalyzePsbtInput, AnalyzePsbtInputMissing,
        AnalyzePsbtInputMissingError, DeriveAddresses, GetNodeAddresses, GetReceivedByLabel,
        ImportMulti, ImportMultiEntry, JoinPsbts, JsonRpcError, ListReceivedByLabel,
        ListReceivedByLabelError, ListUnspent, ListUnspentItem, ListWalletDir, ListWalletDirWallet,
        NodeAddress, UtxoUpdatePsbt,
    },
    v19::{
        Bip9SoftforkInfo, Bip9SoftforkStatistics, Bip9SoftforkStatus, GetBalances,
        GetBalancesError, GetBalancesMine, GetBalancesWatchOnly, GetBlockFilter,
        GetBlockFilterError, GetBlockchainInfoError, GetChainTxStats, GetDescriptorInfo,
        GetMempoolAncestors, GetMempoolAncestorsVerbose, GetMempoolDescendants,
        GetMempoolDescendantsVerbose, GetRpcInfo, MapMempoolEntryError, MempoolEntry,
        MempoolEntryError, MempoolEntryFees, MempoolEntryFeesError, PeerInfo, ScanTxOutSetStart,
        ScanTxOutSetUnspent, SetWalletFlag, Softfork, SoftforkType,
    },
    v20::{GenerateToDescriptor, GetTransactionDetail},
    v21::{
        GenerateBlock, GetIndexInfo, GetIndexInfoName, GetNetworkInfo, ImportDescriptors,
        ImportDescriptorsResult, PsbtBumpFee, PsbtBumpFeeError, Send, SendError, UnloadWallet,
        UpgradeWallet,
    },
    v22::{
        Banned, EnumerateSigners, GetMempoolInfo, ListBanned, ListDescriptors, ScriptPubkey,
        WalletDisplayAddress,
    },
};
