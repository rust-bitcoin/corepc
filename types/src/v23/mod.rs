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
//! | JSON-PRC Method Name               | Returns         | Notes                                  |
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
//! | pruneblockchain                    | returns numeric |                                        |
//! | savemempool                        | version         |                                        |
//! | scantxoutset                       | omitted         | API marked as experimental             |
//! | verifychain                        | returns boolean |                                        |
//! | verifytxoutproof                   | version + model |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Control == section </summary>
//!
//! | JSON-PRC Method Name               | Returns         | Notes                                  |
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
//! | JSON-PRC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | getblocktemplate                   | version + model |                                        |
//! | getmininginfo                      | version         |                                        |
//! | getnetworkhashps                   | returns boolean |                                        |
//! | prioritisetransaction              | returns boolean |                                        |
//! | submitblock                        | returns nothing |                                        |
//! | submitheader                       | return nothing  | TODO                                   |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Network == section </summary>
//!
//! | JSON-PRC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | addnode                            | returns nothing |                                        |
//! | clearbanned                        | returns nothing |                                        |
//! | disconnectnode                     | returns nothing |                                        |
//! | getaddednodeinfo                   | version         |                                        |
//! | getconnectioncount                 | returns numeric |                                        |
//! | getnettotals                       | version         |                                        |
//! | getnetworkinfo                     | version + model |                                        |
//! | getnodeaddresses                   | version         |                                        |
//! | getpeerinfo                        | version         |                                        |
//! | listbanned                         | returns string  |                                        |
//! | ping                               | returns nothing |                                        |
//! | setban                             | returns nothing |                                        |
//! | setnetworkactive                   | returns nothing |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Rawtransactions == section </summary>
//!
//! | JSON-PRC Method Name               | Returns         | Notes                                  |
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
//! | JSON-PRC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | enumeratesigners                   | version + model | TODO                                   |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Util == section </summary>
//!
//! | JSON-PRC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | createmultisig                     | version + model | TODO                                   |
//! | deriveaddresses                    | version + model | TODO                                   |
//! | estimatesmartfee                   | returns nothing | TODO                                   |
//! | getdescriptorinfo                  | version         | TODO                                   |
//! | getindexinfo                       | version         | TODO                                   |
//! | signmessagewithprivkey             | returns string  | TODO                                   |
//! | validateaddress                    | version + model | TODO                                   |
//! | verifymessage                      | returns boolean | TODO                                   |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Wallet == section </summary>
//!
//! | JSON-PRC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | abandontransaction                 | returns nothing |                                        |
//! | abortrescan                        | returns nothing |                                        |
//! | addmultisigaddress                 | version + model | UNTESTED                               |
//! | backupwallet                       | returns nothing |                                        |
//! | bumpfee                            | version + model |                                        |
//! | createwallet                       | version + model |                                        |
//! | dumpprivkey                        | version + model |                                        |
//! | dumpwallet                         | version + model |                                        |
//! | encryptwallet                      | returns nothing |                                        |
//! | getaddressesbylabel                | version + model |                                        |
//! | getaddressinfo                     | version + model | UNTESTED                               |
//! | getbalance                         | version + model |                                        |
//! | getbalances                        | version + model |                                        |
//! | getnewaddress                      | version + model |                                        |
//! | getrawchangeaddress                | version + model |                                        |
//! | getreceivedbyaddress               | version + model | TODO                                   |
//! | getreceivedbylabel                 | version + model | TODO                                   |
//! | gettransaction                     | version + model |                                        |
//! | getunconfirmedbalance              | version + model | UNTESTED                               |
//! | getwalletinfo                      | version + model | UNTESTED                               |
//! | importaddress                      | returns nothing |                                        |
//! | importdescriptors                  | version         | TODO                                   |
//! | importmulti                        | returns nothing |                                        |
//! | importprivkey                      | returns nothing |                                        |
//! | importprunedfunds                  | returns nothing |                                        |
//! | importpubkey                       | returns nothing |                                        |
//! | importwallet                       | returns nothing |                                        |
//! | keypoolrefill                      | returns nothing |                                        |
//! | listaddressgroupings               | version + model | UNTESTED                               |
//! | listdescriptors                    | version + model | TODO                                   |
//! | listlabels                         | version + model | UNTESTED                               |
//! | listlockunspent                    | version + model | UNTESTED                               |
//! | newkeypool                         | version + model | TODO                                   |
//! | psbtbumpfee                        | version + model | TODO                                   |
//! | listreceivedbyaddress              | version + model | UNTESTED                               |
//! | listreceivedbylabel                | version + model | TODO                                   |
//! | listsinceblock                     | version + model | UNTESTED                               |
//! | listtransactions                   | version + model | UNTESTED                               |
//! | listunspent                        | version + model | UNTESTED                               |
//! | listwalletdir                      | version         | TODO                                   |
//! | listwallets                        | version + model | UNTESTED                               |
//! | loadwallet                         | version + model |                                        |
//! | lockunspent                        | returns boolean |                                        |
//! | removeprunedfunds                  | returns nothing |                                        |
//! | rescanblockchain                   | version + model | UNTESTED                               |
//! | restorewallet                      | version + model | TODO                                   |
//! | send                               | version + model | TODO                                   |
//! | sendmany                           | version + model | UNTESTED                               |
//! | sendtoaddress                      | version + model |                                        |
//! | sethdseed                          | returns nothing |                                        |
//! | setlabel                           | returns nothing |                                        |
//! | settxfee                           | returns boolean |                                        |
//! | setwalletflag                      | version         | TODO                                   |
//! | signmessage                        | version + model | UNTESTED                               |
//! | signrawtransactionwithwallet       | version + model |                                        |
//! | unloadwallet                       | returns nothing |                                        |
//! | upgradewallet                      | version         | TODO                                   |
//! | walletcreatefundedpsbt             | version + model | UNTESTED                               |
//! | walletdisplayaddress               | version + model | TODO                                   |
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
//! | JSON-PRC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | getzmqnotifications                | version         | UNTESTED                               |
//!
//! </details>

// JSON-RPC types by API section.
mod blockchain;
mod raw_transactions;

#[doc(inline)]
pub use self::{
    blockchain::SaveMempool,
    raw_transactions::{
        DecodePsbt, DecodePsbtError, GlobalXpub, Proprietary, PsbtInput, PsbtOutput,
    },
};
#[doc(inline)]
pub use crate::{
    v17::{
        AddMultisigAddress, AddMultisigAddressError, AddedNode, AddedNodeAddress,
        AddressInformation, Banned, BumpFee, BumpFeeError, ChainTips, ChainTipsError,
        ChainTipsStatus, CombinePsbt, CombineRawTransaction, ConvertToPsbt, CreatePsbt,
        CreateRawTransaction, CreateWallet, DecodeRawTransaction, DecodeScript, DecodeScriptError,
        DumpPrivKey, DumpWallet, FinalizePsbt, FinalizePsbtError, FundRawTransaction,
        FundRawTransactionError, Generate, GenerateToAddress, GetAddedNodeInfo, GetAddressInfo,
        GetAddressInfoEmbedded, GetAddressInfoEmbeddedError, GetAddressInfoError,
        GetAddressInfoLabel, GetAddressesByLabel, GetBalance, GetBestBlockHash, GetBlockCount,
        GetBlockHash, GetBlockHeader, GetBlockHeaderError, GetBlockHeaderVerbose,
        GetBlockHeaderVerboseError, GetBlockStats, GetBlockStatsError, GetBlockTemplate,
        GetBlockTemplateError, GetBlockVerboseOne, GetBlockVerboseOneError, GetBlockVerboseZero,
        GetChainTips, GetChainTxStats, GetChainTxStatsError, GetDifficulty, GetMemoryInfoStats,
        GetMempoolInfo, GetMempoolInfoError, GetMiningInfo, GetNetTotals, GetNetworkInfo,
        GetNetworkInfoAddress, GetNetworkInfoError, GetNetworkInfoNetwork, GetNewAddress,
        GetPeerInfo, GetRawChangeAddress, GetRawMempool, GetRawMempoolVerbose, GetRawTransaction,
        GetRawTransactionVerbose, GetRawTransactionVerboseError, GetReceivedByAddress,
        GetTransaction, GetTransactionDetail, GetTransactionDetailError, GetTransactionError,
        GetTxOut, GetTxOutError, GetTxOutSetInfo, GetTxOutSetInfoError, GetUnconfirmedBalance,
        GetWalletInfo, GetWalletInfoError, GetZmqNotifications, ListAddressGroupings,
        ListAddressGroupingsError, ListAddressGroupingsItem, ListBanned, ListLabels,
        ListLockUnspent, ListLockUnspentItem, ListLockUnspentItemError, ListReceivedByAddress,
        ListReceivedByAddressError, ListReceivedByAddressItem, ListSinceBlock, ListSinceBlockError,
        ListSinceBlockTransaction, ListSinceBlockTransactionError, ListTransactions,
        ListTransactionsItem, ListTransactionsItemError, ListUnspent, ListUnspentItem,
        ListUnspentItemError, ListWallets, LoadWallet, Locked, PeerInfo, PruneBlockchain,
        RawTransactionError, RawTransactionInput, RawTransactionOutput, RescanBlockchain, SendMany,
        SendRawTransaction, SendToAddress, SignMessage, SignRawTransaction,
        SignRawTransactionError, SoftforkReject, TestMempoolAccept, TransactionCategory,
        UploadTarget, VerifyChain, VerifyTxOutProof, WalletCreateFundedPsbt,
        WalletCreateFundedPsbtError, WalletProcessPsbt, WitnessUtxo,
    },
    v18::{
        ActiveCommand, AnalyzePsbt, AnalyzePsbtError, AnalyzePsbtInput, AnalyzePsbtInputMissing,
        AnalyzePsbtInputMissingError, GetNodeAddresses, GetRpcInfo, JoinPsbts, NodeAddress,
        UtxoUpdatePsbt,
    },
    v19::{
        Bip9SoftforkInfo, Bip9SoftforkStatistics, Bip9SoftforkStatus, GetBalances, GetBalancesMine,
        GetBalancesWatchOnly, GetBlockFilter, GetBlockFilterError, GetBlockchainInfo,
        GetBlockchainInfoError, GetMempoolAncestors, GetMempoolAncestorsVerbose,
        GetMempoolDescendants, GetMempoolDescendantsVerbose, GetMempoolEntry, MapMempoolEntryError,
        MempoolEntry, MempoolEntryError, MempoolEntryFees, MempoolEntryFeesError, Softfork,
        SoftforkType,
    },
    v21::UnloadWallet,
    v22::{Logging, ScriptPubkey},
};
