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
//! | getblockhash                       | version + model |                                        |
//! | getblockheader                     | version + model | Includes additional 'verbose' type     |
//! | getblockstats                      | version + model |                                        |
//! | getchaintips                       | version + model |                                        |
//! | getchaintxstats                    | version + model |                                        |
//! | getdifficulty                      | version + model |                                        |
//! | getmempoolancestors                | version + model | UNTESTED (incl. verbose type)          |
//! | getmempooldescendants              | version + model | UNTESTED (incl. verbose type)          |
//! | getmempoolentry                    | version + model |                                        |
//! | getmempoolinfo                     | version + model |                                        |
//! | getrawmempool                      | version + model |                                        |
//! | gettxout                           | version + model |                                        |
//! | gettxoutproof                      | returns string  |                                        |
//! | gettxoutsetinfo                    | version + model |                                        |
//! | preciousblock                      | returns nothing |                                        |
//! | pruneblockchain                    | returns numeric |                                        |
//! | savemempool                        | returns nothing |                                        |
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
//! | help                               | returns string  |                                        |
//! | logging                            | version         |                                        |
//! | stop                               | returns string  |                                        |
//! | uptime                             | returns numeric |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Generating == section </summary>
//!
//! | JSON-PRC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | generate                           | version + model |                                        |
//! | generatetoaddress                  | version + model |                                        |
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
//! | sendrawtransaction                 | version + model |                                        |
//! | signrawtransaction                 | version + model |                                        |
//! | signrawtransactionwithkey          | version + model |                                        |
//! | testmempoolaccept                  | version + model | UNTESTED                               |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Util == section </summary>
//!
//! | JSON-PRC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | createmultisig                     | version + model | TODO                                   |
//! | estimatesmartfee                   | returns nothing | TODO                                   |
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
//! | getaccount                         | returns nothing |                                        |
//! | getaccountaddress                  | returns nothing |                                        |
//! | getaddressbyaccount                | returns nothing |                                        |
//! | getaddressesbylabel                | version + model |                                        |
//! | getaddressinfo                     | version + model | UNTESTED                               |
//! | getbalance                         | version + model |                                        |
//! | getnewaddress                      | version + model |                                        |
//! | getrawchangeaddress                | version + model |                                        |
//! | getreceivedbyaccount               | returns nothing |                                        |
//! | getreceivedbyaddress               | version + model |                                        |
//! | gettransaction                     | version + model |                                        |
//! | getunconfirmedbalance              | version + model | UNTESTED                               |
//! | getwalletinfo                      | version + model | UNTESTED                               |
//! | importaddress                      | returns nothing |                                        |
//! | importmulti                        | returns nothing |                                        |
//! | importprivkey                      | returns nothing |                                        |
//! | importprunedfunds                  | returns nothing |                                        |
//! | importpubkey                       | returns nothing |                                        |
//! | importwallet                       | returns nothing |                                        |
//! | keypoolrefill                      | returns nothing |                                        |
//! | listaccounts                       | returns nothing |                                        |
//! | listaddressgroupings               | version + model | UNTESTED                               |
//! | listlabels                         | version + model | UNTESTED                               |
//! | listlockunspent                    | version + model | UNTESTED                               |
//! | listreceivedbyaccount              | returns nothing |                                        |
//! | listreceivedbyaddress              | version + model | UNTESTED                               |
//! | listsinceblock                     | version + model | UNTESTED                               |
//! | listtransactions                   | version + model | UNTESTED                               |
//! | listunspent                        | version + model | UNTESTED                               |
//! | listwallets                        | version + model | UNTESTED                               |
//! | loadwallet                         | version + model |                                        |
//! | lockunspent                        | returns boolean |                                        |
//! | move                               | returns boolean |                                        |
//! | removeprunedfunds                  | returns nothing |                                        |
//! | rescanblockchain                   | version + model | UNTESTED                               |
//! | sendfrom                           | returns nothing |                                        |
//! | sendmany                           | version + model | UNTESTED                               |
//! | sendtoaddress                      | version + model |                                        |
//! | setaccount                         | returns nothing |                                        |
//! | sethdseed                          | returns nothing |                                        |
//! | settxfee                           | returns boolean |                                        |
//! | signmessage                        | version + model | UNTESTED                               |
//! | signrawtransactionwithwallet       | version + model |                                        |
//! | unloadwallet                       | returns nothing |                                        |
//! | walletcreatefundedpsbt             | version + model | UNTESTED                               |
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
mod control;
mod generating;
mod mining;
mod network;
pub(crate) mod raw_transactions;
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
        MempoolEntryFees, MempoolEntryFeesError, PruneBlockchain, Softfork, SoftforkReject,
        VerifyChain, VerifyTxOutProof,
    },
    control::{GetMemoryInfoStats, Locked, Logging},
    generating::{Generate, GenerateToAddress},
    mining::{
        BlockTemplateTransaction, BlockTemplateTransactionError, GetBlockTemplate,
        GetBlockTemplateError, GetMiningInfo,
    },
    network::{
        AddedNode, AddedNodeAddress, Banned, GetAddedNodeInfo, GetNetTotals, GetNetworkInfo,
        GetNetworkInfoAddress, GetNetworkInfoError, GetNetworkInfoNetwork, GetPeerInfo, ListBanned,
        PeerInfo, UploadTarget,
    },
    raw_transactions::{
        CombinePsbt, CombineRawTransaction, ConvertToPsbt, CreatePsbt, CreateRawTransaction,
        DecodePsbt, DecodePsbtError, DecodeRawTransaction, DecodeScript, DecodeScriptError,
        FinalizePsbt, FinalizePsbtError, FundRawTransaction, FundRawTransactionError,
        GetRawTransaction, GetRawTransactionVerbose, GetRawTransactionVerboseError,
        MempoolAcceptance, PsbtInput, PsbtInputError, PsbtOutput, PsbtOutputError,
        SendRawTransaction, SignFail, SignFailError, SignRawTransaction, SignRawTransactionError,
        TestMempoolAccept,
    },
    wallet::{
        AddMultisigAddress, AddMultisigAddressError, AddressInformation, BumpFee, BumpFeeError,
        CreateWallet, DumpPrivKey, DumpWallet, GetAddressInfo, GetAddressInfoEmbedded,
        GetAddressInfoEmbeddedError, GetAddressInfoError, GetAddressInfoLabel, GetAddressesByLabel,
        GetBalance, GetNewAddress, GetRawChangeAddress, GetReceivedByAddress, GetTransaction,
        GetTransactionDetail, GetTransactionDetailError, GetTransactionError,
        GetUnconfirmedBalance, GetWalletInfo, GetWalletInfoError, ListAddressGroupings,
        ListAddressGroupingsError, ListAddressGroupingsItem, ListLabels, ListLockUnspent,
        ListLockUnspentItem, ListLockUnspentItemError, ListReceivedByAddress,
        ListReceivedByAddressError, ListReceivedByAddressItem, ListSinceBlock, ListSinceBlockError,
        ListSinceBlockTransaction, ListSinceBlockTransactionError, ListTransactions,
        ListTransactionsItem, ListTransactionsItemError, ListUnspent, ListUnspentItem,
        ListUnspentItemError, ListWallets, LoadWallet, RescanBlockchain, SendMany, SendToAddress,
        SignMessage, TransactionCategory, WalletCreateFundedPsbt, WalletCreateFundedPsbtError,
        WalletProcessPsbt,
    },
    zmq::GetZmqNotifications,
};
#[doc(inline)]
pub use crate::psbt::{
    Bip32Deriv, Bip32DerivError, FinalScript, InputKeySource, PartialSignatureError, PsbtScript,
    RawTransaction, RawTransactionError, RawTransactionInput, RawTransactionInputError,
    RawTransactionOutput, RawTransactionOutputError, WitnessUtxo, WitnessUtxoError,
};
