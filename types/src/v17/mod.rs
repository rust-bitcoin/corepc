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
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
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
//! | getmempoolancestors                | version + model |                                        |
//! | getmempooldescendants              | version + model |                                        |
//! | getmempoolentry                    | version + model |                                        |
//! | getmempoolinfo                     | version + model |                                        |
//! | getrawmempool                      | version + model |                                        |
//! | gettxout                           | version + model |                                        |
//! | gettxoutproof                      | returns string  |                                        |
//! | gettxoutsetinfo                    | version + model |                                        |
//! | preciousblock                      | returns nothing |                                        |
//! | pruneblockchain                    | version         |                                        |
//! | savemempool                        | returns nothing |                                        |
//! | verifychain                        | version         |                                        |
//! | scantxoutset                       | version + model | API marked as experimental             |
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
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | generate                           | version + model |                                        |
//! | generatetoaddress                  | version + model |                                        |
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
//! | combinepsbt                        | version + model |                                        |
//! | combinerawtransaction              | version + model |                                        |
//! | converttopsbt                      | version + model |                                        |
//! | createpsbt                         | version + model |                                        |
//! | createrawtransaction               | version + model |                                        |
//! | decodepsbt                         | version + model |                                        |
//! | decoderawtransaction               | version + model |                                        |
//! | decodescript                       | version + model |                                        |
//! | finalizepsbt                       | version + model |                                        |
//! | fundrawtransaction                 | version + model |                                        |
//! | getrawtransaction                  | version + model | Includes additional 'verbose' type     |
//! | sendrawtransaction                 | version + model |                                        |
//! | signrawtransaction                 | version + model |                                        |
//! | signrawtransactionwithkey          | version + model |                                        |
//! | testmempoolaccept                  | version + model |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Util == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | createmultisig                     | version + model |                                        |
//! | estimatesmartfee                   | version + model |                                        |
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
//! | addmultisigaddress                 | version + model |                                        |
//! | backupwallet                       | returns nothing |                                        |
//! | bumpfee                            | version + model |                                        |
//! | createwallet                       | version + model |                                        |
//! | dumpprivkey                        | version + model |                                        |
//! | dumpwallet                         | version         |                                        |
//! | encryptwallet                      | version         |                                        |
//! | getaccount                         | returns nothing |                                        |
//! | getaccountaddress                  | returns nothing |                                        |
//! | getaddressbyaccount                | returns nothing |                                        |
//! | getaddressesbylabel                | version + model |                                        |
//! | getaddressinfo                     | version + model |                                        |
//! | getbalance                         | version + model |                                        |
//! | getnewaddress                      | version + model |                                        |
//! | getrawchangeaddress                | version + model |                                        |
//! | getreceivedbyaccount               | returns nothing |                                        |
//! | getreceivedbyaddress               | version + model |                                        |
//! | gettransaction                     | version + model |                                        |
//! | getunconfirmedbalance              | version + model |                                        |
//! | getwalletinfo                      | version + model |                                        |
//! | importaddress                      | returns nothing |                                        |
//! | importmulti                        | version         |                                        |
//! | importprivkey                      | returns nothing |                                        |
//! | importprunedfunds                  | returns nothing |                                        |
//! | importpubkey                       | returns nothing |                                        |
//! | importwallet                       | returns nothing |                                        |
//! | keypoolrefill                      | returns nothing |                                        |
//! | listaccounts                       | returns nothing |                                        |
//! | listaddressgroupings               | version + model |                                        |
//! | listlabels                         | version         |                                        |
//! | listlockunspent                    | version + model |                                        |
//! | listreceivedbyaccount              | returns nothing |                                        |
//! | listreceivedbyaddress              | version + model |                                        |
//! | listsinceblock                     | version + model |                                        |
//! | listtransactions                   | version + model |                                        |
//! | listunspent                        | version + model |                                        |
//! | listwallets                        | version + model |                                        |
//! | loadwallet                         | version + model |                                        |
//! | lockunspent                        | version         |                                        |
//! | move                               | returns boolean |                                        |
//! | removeprunedfunds                  | returns nothing |                                        |
//! | rescanblockchain                   | version + model |                                        |
//! | sendfrom                           | returns nothing |                                        |
//! | sendmany                           | version + model |                                        |
//! | sendtoaddress                      | version + model |                                        |
//! | setaccount                         | returns nothing |                                        |
//! | sethdseed                          | returns nothing |                                        |
//! | settxfee                           | version         |                                        |
//! | signmessage                        | version + model |                                        |
//! | signrawtransactionwithwallet       | version + model |                                        |
//! | unloadwallet                       | returns nothing |                                        |
//! | walletcreatefundedpsbt             | version + model |                                        |
//! | walletlock                         | returns nothing |                                        |
//! | walletpassphrase                   | returns nothing |                                        |
//! | walletpassphrasechange             | returns nothing |                                        |
//! | walletprocesspsbt                  | version + model |                                        |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Zmq == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | getzmqnotifications                | version         |                                        |
//!
//! </details>

// JSON-RPC types by API section.
mod blockchain;
mod control;
mod generating;
mod hidden;
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
        MempoolEntryFees, MempoolEntryFeesError, PruneBlockchain, ScanTxOutSetAbort,
        ScanTxOutSetError, ScanTxOutSetStart, ScanTxOutSetStatus, ScanTxOutSetUnspent, Softfork,
        SoftforkReject, VerifyChain, VerifyTxOutProof,
    },
    control::{GetMemoryInfoStats, Locked, Logging},
    generating::{Generate, GenerateToAddress},
    hidden::{
        WaitForBlock, WaitForBlockError, WaitForBlockHeight, WaitForBlockHeightError,
        WaitForNewBlock, WaitForNewBlockError,
    },
    mining::{
        BlockTemplateTransaction, BlockTemplateTransactionError, GetBlockTemplate,
        GetBlockTemplateError, GetMiningInfo,
    },
    network::{
        AddedNode, AddedNodeAddress, Banned, GetAddedNodeInfo, GetConnectionCount, GetNetTotals,
        GetNetworkInfo, GetNetworkInfoAddress, GetNetworkInfoError, GetNetworkInfoNetwork,
        GetPeerInfo, ListBanned, PeerInfo, SetNetworkActive, UploadTarget,
    },
    raw_transactions::{
        CombinePsbt, CombineRawTransaction, ConvertToPsbt, CreatePsbt, CreateRawTransaction,
        DecodePsbt, DecodePsbtError, DecodeRawTransaction, DecodeScript, DecodeScriptError,
        DecodeScriptSegwit, FinalizePsbt, FinalizePsbtError, FundRawTransaction,
        FundRawTransactionError, GetRawTransaction, GetRawTransactionVerbose,
        GetRawTransactionVerboseError, MempoolAcceptance, PsbtInput, PsbtInputError, PsbtOutput,
        PsbtOutputError, SendRawTransaction, SignFail, SignFailError, SignRawTransaction,
        SignRawTransactionError, SignRawTransactionWithKey, TestMempoolAccept,
    },
    util::{
        CreateMultisig, CreateMultisigError, EstimateSmartFee, SignMessageWithPrivKey,
        ValidateAddress, ValidateAddressError, VerifyMessage,
    },
    wallet::{
        AbortRescan, AddMultisigAddress, AddMultisigAddressError, AddressInformation,
        AddressPurpose, Bip125Replaceable, BumpFee, BumpFeeError, CreateWallet, DumpPrivKey,
        DumpWallet, EncryptWallet, GetAddressInfo, GetAddressInfoEmbedded,
        GetAddressInfoEmbeddedError, GetAddressInfoError, GetAddressInfoLabel, GetAddressesByLabel,
        GetBalance, GetNewAddress, GetRawChangeAddress, GetReceivedByAddress, GetTransaction,
        GetTransactionDetail, GetTransactionDetailError, GetTransactionError,
        GetUnconfirmedBalance, GetWalletInfo, GetWalletInfoError, ImportMulti, ImportMultiEntry,
        JsonRpcError, ListAddressGroupings, ListAddressGroupingsError, ListAddressGroupingsItem,
        ListLabels, ListLockUnspent, ListLockUnspentItem, ListLockUnspentItemError,
        ListReceivedByAddress, ListReceivedByAddressError, ListReceivedByAddressItem,
        ListSinceBlock, ListSinceBlockError, ListTransactions, ListUnspent, ListUnspentItem,
        ListUnspentItemError, ListWallets, LoadWallet, LockUnspent, RescanBlockchain, ScriptType,
        SendMany, SendToAddress, SetTxFee, SignMessage, SignRawTransactionWithWallet,
        TransactionCategory, TransactionItem, TransactionItemError, WalletCreateFundedPsbt,
        WalletCreateFundedPsbtError, WalletProcessPsbt,
    },
    zmq::GetZmqNotifications,
};
#[doc(inline)]
pub use crate::psbt::{
    Bip32Deriv, Bip32DerivError, FinalScript, InputKeySource, PartialSignatureError, PsbtScript,
    RawTransaction, RawTransactionError, RawTransactionInput, RawTransactionInputError,
    RawTransactionOutput, RawTransactionOutputError, WitnessUtxo, WitnessUtxoError,
};
#[doc(inline)]
pub use crate::NumericError;
