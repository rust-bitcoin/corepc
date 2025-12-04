// SPDX-License-Identifier: CC0-1.0

//! # JSON-RPC types for Bitcoin Core `v0.18`
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
//! | getrawmempool                      | version + model | Includes additional 'verbose' type     |
//! | gettxout                           | version + model |                                        |
//! | gettxoutproof                      | returns string  |                                        |
//! | gettxoutsetinfo                    | version + model |                                        |
//! | preciousblock                      | returns nothing |                                        |
//! | pruneblockchain                    | version         |                                        |
//! | savemempool                        | returns nothing |                                        |
//! | verifychain                        | version         |                                        |
//! | scantxoutset                       | version + model | API marked as experimental             |
//! | verifychain                        | returns boolean |                                        |
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
//! | getrpcinfo                         | version         |                                        |
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
//! | generate                           | omitted         | Deprecated                             |
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
//! | getnodeaddresses                   | version         |                                        |
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
//! | finalizepsbt                       | version + model |                                        |
//! | fundrawtransaction                 | version + model |                                        |
//! | getrawtransaction                  | version + model | Includes additional 'verbose' type     |
//! | joinpsbts                          | version + model |                                        |
//! | sendrawtransaction                 | version + model |                                        |
//! | signrawtransactionwithkey          | version + model |                                        |
//! | testmempoolaccept                  | version + model |                                        |
//! | utxoupdatepsbt                     | version + model |                                        |
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
//! | getaddressesbylabel                | version + model |                                        |
//! | getaddressinfo                     | version + model |                                        |
//! | getbalance                         | version + model |                                        |
//! | getnewaddress                      | version + model |                                        |
//! | getrawchangeaddress                | version + model |                                        |
//! | getreceivedbyaddress               | version + model |                                        |
//! | getreceivedbylabel                 | version + model |                                        |
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
//! | listaddressgroupings               | version + model |                                        |
//! | listlabels                         | version         |                                        |
//! | listlockunspent                    | version + model |                                        |
//! | listreceivedbyaddress              | version + model |                                        |
//! | listreceivedbylabel                | version + model |                                        |
//! | listsinceblock                     | version + model |                                        |
//! | listtransactions                   | version + model |                                        |
//! | listunspent                        | version + model |                                        |
//! | listwalletdir                      | version         |                                        |
//! | listwallets                        | version + model |                                        |
//! | loadwallet                         | version + model |                                        |
//! | lockunspent                        | version         |                                        |
//! | removeprunedfunds                  | returns nothing |                                        |
//! | rescanblockchain                   | version + model |                                        |
//! | sendmany                           | version + model |                                        |
//! | sendtoaddress                      | version + model |                                        |
//! | sethdseed                          | returns nothing |                                        |
//! | setlabel                           | returns nothing |                                        |
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
mod network;
mod raw_transactions;
mod util;
mod wallet;
mod zmq;

#[doc(inline)]
pub use self::{
    blockchain::{
        GetMempoolAncestors, GetMempoolAncestorsVerbose, GetMempoolDescendants,
        GetMempoolDescendantsVerbose, GetMempoolEntry, GetRawMempool, GetRawMempoolVerbose,
        MempoolEntry, ScanTxOutSetStart, ScanTxOutSetUnspent,
    },
    control::{ActiveCommand, GetRpcInfo},
    network::{GetNodeAddresses, GetPeerInfo, NodeAddress, PeerInfo},
    raw_transactions::{
        AnalyzePsbt, AnalyzePsbtError, AnalyzePsbtInput, AnalyzePsbtInputMissing,
        AnalyzePsbtInputMissingError, JoinPsbts, UtxoUpdatePsbt,
    },
    util::{DeriveAddresses, GetDescriptorInfo},
    wallet::{
        GetAddressInfo, GetAddressInfoEmbedded, GetAddressInfoError, GetReceivedByLabel,
        GetWalletInfo, ImportMulti, ImportMultiEntry, JsonRpcError, ListReceivedByAddress,
        ListReceivedByAddressItem, ListReceivedByLabel, ListReceivedByLabelError,
        ListReceivedByLabelItem, ListUnspent, ListUnspentItem, ListWalletDir, ListWalletDirWallet,
    },
    zmq::GetZmqNotifications,
};
#[doc(inline)]
pub use crate::v17::{
    AbortRescan, AddMultisigAddress, AddMultisigAddressError, AddedNode, AddedNodeAddress,
    AddressInformation, AddressPurpose, Banned, Bip125Replaceable, Bip32Deriv, Bip32DerivError,
    Bip9Softfork, Bip9SoftforkStatus, BlockTemplateTransaction, BlockTemplateTransactionError,
    BumpFee, BumpFeeError, ChainTips, ChainTipsError, ChainTipsStatus, CombinePsbt,
    CombineRawTransaction, ConvertToPsbt, CreateMultisig, CreateMultisigError, CreatePsbt,
    CreateRawTransaction, CreateWallet, DecodePsbt, DecodePsbtError, DecodeRawTransaction,
    DecodeScript, DecodeScriptError, DecodeScriptSegwit, DumpPrivKey, DumpWallet, EncryptWallet,
    EstimateSmartFee, FinalizePsbt, FinalizePsbtError, FundRawTransaction, FundRawTransactionError,
    Generate, GenerateToAddress, GetAddedNodeInfo, GetAddressInfoEmbeddedError,
    GetAddressInfoLabel, GetAddressesByLabel, GetBalance, GetBestBlockHash, GetBlockCount,
    GetBlockHash, GetBlockHeader, GetBlockHeaderError, GetBlockHeaderVerbose,
    GetBlockHeaderVerboseError, GetBlockStats, GetBlockStatsError, GetBlockTemplate,
    GetBlockTemplateError, GetBlockVerboseOne, GetBlockVerboseOneError, GetBlockVerboseZero,
    GetBlockchainInfo, GetBlockchainInfoError, GetChainTips, GetChainTxStats, GetChainTxStatsError,
    GetConnectionCount, GetDifficulty, GetMemoryInfoStats, GetMempoolInfo, GetMempoolInfoError,
    GetMiningInfo, GetNetTotals, GetNetworkInfo, GetNetworkInfoAddress, GetNetworkInfoError,
    GetNetworkInfoNetwork, GetNewAddress, GetRawChangeAddress, GetRawTransaction,
    GetRawTransactionVerbose, GetRawTransactionVerboseError, GetReceivedByAddress, GetTransaction,
    GetTransactionDetail, GetTransactionDetailError, GetTransactionError, GetTxOut, GetTxOutError,
    GetTxOutSetInfo, GetTxOutSetInfoError, GetUnconfirmedBalance, GetWalletInfoError,
    ListAddressGroupings, ListAddressGroupingsError, ListAddressGroupingsItem, ListBanned,
    ListLabels, ListLockUnspent, ListLockUnspentItem, ListLockUnspentItemError,
    ListReceivedByAddressError, ListSinceBlock, ListSinceBlockError, ListTransactions,
    ListUnspentItemError, ListWallets, LoadWallet, LockUnspent, Locked, Logging,
    MapMempoolEntryError, MempoolAcceptance, MempoolEntryError, MempoolEntryFees,
    MempoolEntryFeesError, NumericError, PartialSignatureError, PruneBlockchain, PsbtInput,
    PsbtInputError, PsbtOutput, PsbtOutputError, PsbtScript, RawTransaction, RawTransactionError,
    RawTransactionInput, RawTransactionOutput, RescanBlockchain, ScanTxOutSetAbort,
    ScanTxOutSetError, ScanTxOutSetStatus, ScriptType, SendMany, SendRawTransaction, SendToAddress,
    SetNetworkActive, SetTxFee, SignFail, SignFailError, SignMessage, SignMessageWithPrivKey,
    SignRawTransaction, SignRawTransactionError, SignRawTransactionWithKey,
    SignRawTransactionWithWallet, Softfork, SoftforkReject, TestMempoolAccept, TransactionCategory,
    TransactionItem, TransactionItemError, UploadTarget, ValidateAddress, ValidateAddressError,
    VerifyChain, VerifyMessage, VerifyTxOutProof, WaitForBlock, WaitForBlockError,
    WaitForBlockHeight, WaitForBlockHeightError, WaitForNewBlock, WaitForNewBlockError,
    WalletCreateFundedPsbt, WalletCreateFundedPsbtError, WalletProcessPsbt, WitnessUtxo,
    WitnessUtxoError,
};
