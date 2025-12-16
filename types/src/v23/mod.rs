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
//! | getblockfilter                     | version + model |                                        |
//! | getblockfrompeer                   | returns nothing |                                        |
//! | getblockhash                       | version + model |                                        |
//! | getblockheader                     | version + model | Includes additional 'verbose' type     |
//! | getblockstats                      | version + model |                                        |
//! | getchaintips                       | version + model |                                        |
//! | getchaintxstats                    | version + model |                                        |
//! | getdeploymentinfo                  | version + model |                                        |
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
//! | savemempool                        | version         |                                        |
//! | scantxoutset                       | omitted         | API marked as experimental             |
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
//! | getrpcinfo                         | version         |                                        |
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
//! <summary> Methods from the == Signer == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | enumeratesigners                   | version         |                                        |
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
//! | getbalances                        | version + model |                                        |
//! | getnewaddress                      | version + model |                                        |
//! | getrawchangeaddress                | version + model |                                        |
//! | getreceivedbyaddress               | version + model |                                        |
//! | getreceivedbylabel                 | version + model |                                        |
//! | gettransaction                     | version + model |                                        |
//! | getunconfirmedbalance              | version + model |                                        |
//! | getwalletinfo                      | version + model |                                        |
//! | importaddress                      | returns nothing |                                        |
//! | importdescriptors                  | version         |                                        |
//! | importmulti                        | version         |                                        |
//! | importprivkey                      | returns nothing |                                        |
//! | importprunedfunds                  | returns nothing |                                        |
//! | importpubkey                       | returns nothing |                                        |
//! | importwallet                       | returns nothing |                                        |
//! | keypoolrefill                      | returns nothing |                                        |
//! | listaddressgroupings               | version + model |                                        |
//! | listdescriptors                    | version         |                                        |
//! | listlabels                         | version         |                                        |
//! | listlockunspent                    | version + model |                                        |
//! | newkeypool                         | returns nothing |                                        |
//! | psbtbumpfee                        | version + model |                                        |
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
//! | restorewallet                      | version         |                                        |
//! | send                               | version + model |                                        |
//! | sendmany                           | version + model |                                        |
//! | sendtoaddress                      | version + model |                                        |
//! | sethdseed                          | returns nothing |                                        |
//! | setlabel                           | returns nothing |                                        |
//! | settxfee                           | version         |                                        |
//! | setwalletflag                      | version         |                                        |
//! | signmessage                        | version + model |                                        |
//! | signrawtransactionwithwallet       | version + model |                                        |
//! | unloadwallet                       | returns nothing |                                        |
//! | upgradewallet                      | version         |                                        |
//! | walletcreatefundedpsbt             | version + model |                                        |
//! | walletdisplayaddress               | version + model | UNTESTED                               |
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

#[doc(inline)]
pub use self::{
    blockchain::{
        Bip9Info, Bip9Statistics, DeploymentInfo, GetBlockchainInfo, GetDeploymentInfo,
        GetDeploymentInfoError, GetMempoolAncestors, GetMempoolAncestorsVerbose,
        GetMempoolDescendants, GetMempoolDescendantsVerbose, GetMempoolEntry, GetRawMempool,
        GetRawMempoolVerbose, MempoolEntry, SaveMempool,
    },
    control::Logging,
    network::{GetPeerInfo, PeerInfo},
    raw_transactions::{
        DecodePsbt, DecodePsbtError, DecodeScript, DecodeScriptError, DecodeScriptSegwit,
        GlobalXpub, GlobalXpubError, Proprietary, PsbtInput, PsbtInputError, PsbtOutput,
        PsbtOutputError,
    },
    util::CreateMultisig,
    wallet::{
        AddMultisigAddress, GetTransaction, GetTransactionError, GetWalletInfo,
        GetWalletInfoScanning, ListSinceBlock, ListSinceBlockError, ListTransactions,
        RestoreWallet, TransactionItem, TransactionItemError,
    },
};
#[doc(inline)]
pub use crate::{
    v17::{
        AbortRescan, AddMultisigAddressError, AddedNode, AddedNodeAddress, AddressInformation,
        AddressPurpose, Bip125Replaceable, Bip32DerivError, BlockTemplateTransaction,
        BlockTemplateTransactionError, BumpFee, BumpFeeError, ChainTips, ChainTipsError,
        ChainTipsStatus, CombinePsbt, CombineRawTransaction, ConvertToPsbt, CreateMultisigError,
        CreatePsbt, CreateRawTransaction, CreateWallet, DecodeRawTransaction, DumpPrivKey,
        DumpWallet, EncryptWallet, EstimateRawFee, EstimateRawFeeError, EstimateSmartFee,
        FinalizePsbt, FinalizePsbtError, FundRawTransaction, FundRawTransactionError, Generate,
        GenerateToAddress, GetAddedNodeInfo, GetAddressInfoEmbeddedError, GetAddressesByLabel,
        GetBalance, GetBestBlockHash, GetBlockCount, GetBlockHash, GetBlockHeader,
        GetBlockHeaderError, GetBlockHeaderVerbose, GetBlockHeaderVerboseError, GetBlockStats,
        GetBlockStatsError, GetBlockTemplate, GetBlockTemplateError, GetBlockVerboseOne,
        GetBlockVerboseOneError, GetBlockVerboseZero, GetChainTips, GetChainTxStatsError,
        GetConnectionCount, GetDifficulty, GetMemoryInfoStats, GetMempoolInfoError, GetMiningInfo,
        GetNetTotals, GetNetworkInfoAddress, GetNetworkInfoError, GetNetworkInfoNetwork,
        GetNewAddress, GetRawChangeAddress, GetRawTransaction, GetRawTransactionVerbose,
        GetRawTransactionVerboseError, GetReceivedByAddress, GetTransactionDetailError, GetTxOut,
        GetTxOutError, GetTxOutSetInfo, GetTxOutSetInfoError, GetUnconfirmedBalance,
        GetWalletInfoError, ListAddressGroupings, ListAddressGroupingsError,
        ListAddressGroupingsItem, ListLabels, ListLockUnspent, ListLockUnspentItem,
        ListLockUnspentItemError, ListReceivedByAddressError, ListUnspentItemError, ListWallets,
        LoadWallet, LockUnspent, Locked, NumericError, PartialSignatureError, PruneBlockchain,
        RawFeeDetail, RawFeeRange, RawTransactionError, RawTransactionInput, RawTransactionOutput,
        RescanBlockchain, ScriptType, SendRawTransaction, SendToAddress, SetNetworkActive,
        SetTxFee, SignFail, SignFailError, SignMessage, SignMessageWithPrivKey, SignRawTransaction,
        SignRawTransactionError, SignRawTransactionWithKey, SignRawTransactionWithWallet,
        SoftforkReject, TransactionCategory, UploadTarget, ValidateAddress, ValidateAddressError,
        VerifyChain, VerifyMessage, VerifyTxOutProof, WaitForBlock, WaitForBlockError,
        WaitForBlockHeight, WaitForBlockHeightError, WaitForNewBlock, WaitForNewBlockError,
        WalletCreateFundedPsbt, WalletCreateFundedPsbtError, WalletProcessPsbt, WitnessUtxo,
        WitnessUtxoError,
    },
    v18::{
        ActiveCommand, AnalyzePsbt, AnalyzePsbtError, AnalyzePsbtInput, AnalyzePsbtInputMissing,
        AnalyzePsbtInputMissingError, DeriveAddresses, GetAddressInfoError, GetReceivedByLabel,
        GetZmqNotifications, ImportMulti, ImportMultiEntry, JoinPsbts, JsonRpcError,
        ListReceivedByAddress, ListReceivedByAddressItem, ListReceivedByLabel,
        ListReceivedByLabelError, ListReceivedByLabelItem, ListUnspent, ListUnspentItem,
        ListWalletDir, ListWalletDirWallet, UtxoUpdatePsbt,
    },
    v19::{
        Bip9SoftforkInfo, Bip9SoftforkStatistics, Bip9SoftforkStatus, GetBalances,
        GetBalancesError, GetBalancesMine, GetBalancesWatchOnly, GetBlockFilter,
        GetBlockFilterError, GetBlockchainInfoError, GetChainTxStats, GetDescriptorInfo,
        GetRpcInfo, MapMempoolEntryError, MempoolEntryError, MempoolEntryFees,
        MempoolEntryFeesError, SetWalletFlag, Softfork, SoftforkType,
    },
    v20::{GenerateToDescriptor, GetTransactionDetail},
    v21::{
        AddPeerAddress, GenerateBlock, GetIndexInfo, GetIndexInfoName, GetNetworkInfo,
        ImportDescriptors, ImportDescriptorsResult, PsbtBumpFee, PsbtBumpFeeError, Send, SendError,
        SendMany, SendManyVerbose, UnloadWallet, UpgradeWallet,
    },
    v22::{
        AddConnection, Banned, DescriptorInfo, EnumerateSigners, GetAddressInfo,
        GetAddressInfoEmbedded, GetMempoolInfo, GetNodeAddresses, ListBanned, ListDescriptors,
        MempoolAcceptance, MempoolAcceptanceError, MempoolAcceptanceFees, NodeAddress,
        ScriptPubkey, Signers, TestMempoolAccept, TestMempoolAcceptError, WalletDisplayAddress,
    },
};
