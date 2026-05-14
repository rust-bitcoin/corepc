// SPDX-License-Identifier: CC0-1.0

//! # JSON-RPC types for Bitcoin Core `v31`
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
//! | dumptxoutset                       | version + model | TODO                                   |
//! | getbestblockhash                   | version + model | TODO                                   |
//! | getblock                           | version + model | TODO                                   |
//! | getblockchaininfo                  | version + model | TODO                                   |
//! | getblockcount                      | version + model | TODO                                   |
//! | getblockfilter                     | version + model | TODO                                   |
//! | getblockfrompeer                   | returns nothing | TODO                                   |
//! | getblockhash                       | version + model | TODO                                   |
//! | getblockheader                     | version + model | TODO                                   |
//! | getblockstats                      | version + model | TODO                                   |
//! | getchainstates                     | version + model | TODO                                   |
//! | getchaintips                       | version + model | TODO                                   |
//! | getchaintxstats                    | version + model | TODO                                   |
//! | getdeploymentinfo                  | version + model | TODO                                   |
//! | getdescriptoractivity              | version + model | TODO                                   |
//! | getdifficulty                      | version + model | TODO                                   |
//! | getmempoolancestors                | version + model | TODO                                   |
//! | getmempoolcluster                  | version + model | TODO                                   |
//! | getmempooldescendants              | version + model | TODO                                   |
//! | getmempoolentry                    | version + model | TODO                                   |
//! | getmempoolfeeratediagram           | version + model | TODO                                   |
//! | getmempoolinfo                     | version + model | TODO                                   |
//! | getrawmempool                      | version + model | TODO                                   |
//! | gettxout                           | version + model | TODO                                   |
//! | gettxoutproof                      | returns string  | TODO                                   |
//! | gettxoutsetinfo                    | version + model | TODO                                   |
//! | gettxspendingprevout               | version + model | TODO                                   |
//! | importmempool                      | returns nothing | TODO                                   |
//! | loadtxoutset                       | version + model | TODO                                   |
//! | preciousblock                      | returns nothing | TODO                                   |
//! | pruneblockchain                    | version         | TODO                                   |
//! | savemempool                        | version         | TODO                                   |
//! | scanblocks                         | version + model | TODO                                   |
//! | scantxoutset                       | version + model | TODO                                   |
//! | verifychain                        | version         | TODO                                   |
//! | verifytxoutproof                   | version + model | TODO                                   |
//! | waitforblock                       | version + model | TODO                                   |
//! | waitforblockheight                 | version + model | TODO                                   |
//! | waitfornewblock                    | version + model | TODO                                   |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Control == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | getmemoryinfo                      | version         | TODO                                   |
//! | getrpcinfo                         | version         | TODO                                   |
//! | help                               | returns string  | TODO                                   |
//! | logging                            | version         | TODO                                   |
//! | stop                               | returns string  | TODO                                   |
//! | uptime                             | returns numeric | TODO                                   |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Mining == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | getblocktemplate                   | version + model | TODO                                   |
//! | getmininginfo                      | version + model | TODO                                   |
//! | getnetworkhashps                   | returns boolean | TODO                                   |
//! | getprioritisedtransactions         | version + model | TODO                                   |
//! | prioritisetransaction              | returns boolean | TODO                                   |
//! | submitblock                        | returns nothing | TODO                                   |
//! | submitheader                       | returns nothing | TODO                                   |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Network == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | abortprivatebroadcast              | version + model | TODO                                   |
//! | addnode                            | returns nothing | TODO                                   |
//! | clearbanned                        | returns nothing | TODO                                   |
//! | disconnectnode                     | returns nothing | TODO                                   |
//! | getaddednodeinfo                   | version         | TODO                                   |
//! | getaddrmaninfo                     | version         | TODO                                   |
//! | getconnectioncount                 | version         | TODO                                   |
//! | getnettotals                       | version         | TODO                                   |
//! | getnetworkinfo                     | version + model | TODO                                   |
//! | getnodeaddresses                   | version         | TODO                                   |
//! | getpeerinfo                        | version         | TODO                                   |
//! | getprivatebroadcastinfo            | version + model | TODO                                   |
//! | listbanned                         | version         | TODO                                   |
//! | ping                               | returns nothing | TODO                                   |
//! | setban                             | returns nothing | TODO                                   |
//! | setnetworkactive                   | version         | TODO                                   |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Rawtransactions == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | analyzepsbt                        | version + model | TODO                                   |
//! | combinepsbt                        | version + model | TODO                                   |
//! | combinerawtransaction              | version + model | TODO                                   |
//! | converttopsbt                      | version + model | TODO                                   |
//! | createpsbt                         | version + model | TODO                                   |
//! | createrawtransaction               | version + model | TODO                                   |
//! | decodepsbt                         | version + model | TODO                                   |
//! | descriptorprocesspsbt              | returns boolean | TODO                                   |
//! | decoderawtransaction               | version + model | TODO                                   |
//! | decodescript                       | version + model | TODO                                   |
//! | finalizepsbt                       | version + model | TODO                                   |
//! | fundrawtransaction                 | version + model | TODO                                   |
//! | getrawtransaction                  | version + model | TODO                                   |
//! | joinpsbts                          | version + model | TODO                                   |
//! | sendrawtransaction                 | version + model | TODO                                   |
//! | signrawtransactionwithkey          | version + model | TODO                                   |
//! | submitpackage                      | version + model | TODO                                   |
//! | testmempoolaccept                  | version + model | TODO                                   |
//! | utxoupdatepsbt                     | version + model | TODO                                   |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Signer == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | enumeratesigners                   | version         | TODO                                   |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Util == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | createmultisig                     | version + model | TODO                                   |
//! | deriveaddresses                    | version + model | TODO                                   |
//! | estimatesmartfee                   | version + model | TODO                                   |
//! | getdescriptorinfo                  | version         | TODO                                   |
//! | getindexinfo                       | version         | TODO                                   |
//! | signmessagewithprivkey             | version + model | TODO                                   |
//! | validateaddress                    | version + model | TODO                                   |
//! | verifymessage                      | version         | TODO                                   |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Wallet == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | abandontransaction                 | returns nothing | TODO                                   |
//! | abortrescan                        | version         | TODO                                   |
//! | backupwallet                       | returns nothing | TODO                                   |
//! | bumpfee                            | version + model | TODO                                   |
//! | createwallet                       | version + model | TODO                                   |
//! | createwalletdescriptor             | version         | TODO                                   |
//! | encryptwallet                      | version         | TODO                                   |
//! | getaddressesbylabel                | version + model | TODO                                   |
//! | getaddressinfo                     | version + model | TODO                                   |
//! | getbalance                         | version + model | TODO                                   |
//! | getbalances                        | version + model | TODO                                   |
//! | gethdkeys                          | version + model | TODO                                   |
//! | getnewaddress                      | version + model | TODO                                   |
//! | getrawchangeaddress                | version + model | TODO                                   |
//! | getreceivedbyaddress               | version + model | TODO                                   |
//! | getreceivedbylabel                 | version + model | TODO                                   |
//! | gettransaction                     | version + model | TODO                                   |
//! | getwalletinfo                      | version + model | TODO                                   |
//! | importdescriptors                  | version         | TODO                                   |
//! | importprunedfunds                  | returns nothing | TODO                                   |
//! | keypoolrefill                      | returns nothing | TODO                                   |
//! | listaddressgroupings               | version + model | TODO                                   |
//! | listdescriptors                    | version         | TODO                                   |
//! | listlabels                         | version         | TODO                                   |
//! | listlockunspent                    | version + model | TODO                                   |
//! | migratewallet                      | version         | TODO                                   |
//! | psbtbumpfee                        | version + model | TODO                                   |
//! | listreceivedbyaddress              | version + model | TODO                                   |
//! | listreceivedbylabel                | version + model | TODO                                   |
//! | listsinceblock                     | version + model | TODO                                   |
//! | listtransactions                   | version + model | TODO                                   |
//! | listunspent                        | version + model | TODO                                   |
//! | listwalletdir                      | version         | TODO                                   |
//! | listwallets                        | version + model | TODO                                   |
//! | loadwallet                         | version + model | TODO                                   |
//! | lockunspent                        | version         | TODO                                   |
//! | removeprunedfunds                  | returns nothing | TODO                                   |
//! | rescanblockchain                   | version + model | TODO                                   |
//! | restorewallet                      | version         | TODO                                   |
//! | send                               | version + model | TODO                                   |
//! | sendall                            | version + model | TODO                                   |
//! | sendmany                           | version + model | TODO                                   |
//! | sendtoaddress                      | version + model | TODO                                   |
//! | setlabel                           | returns nothing | TODO                                   |
//! | setwalletflag                      | version         | TODO                                   |
//! | signmessage                        | version + model | TODO                                   |
//! | signrawtransactionwithwallet       | version + model | TODO                                   |
//! | simulaterawtransaction             | version + model | TODO                                   |
//! | unloadwallet                       | returns nothing | TODO                                   |
//! | walletcreatefundedpsbt             | version + model | TODO                                   |
//! | walletdisplayaddress               | version + model | TODO                                   |
//! | walletlock                         | returns nothing | TODO                                   |
//! | walletpassphrase                   | returns nothing | TODO                                   |
//! | walletpassphrasechange             | returns nothing | TODO                                   |
//! | walletprocesspsbt                  | version + model | TODO                                   |
//!
//! </details>
//!
//! <details>
//! <summary> Methods from the == Zmq == section </summary>
//!
//! | JSON-RPC Method Name               | Returns         | Notes                                  |
//! |:-----------------------------------|:---------------:|:--------------------------------------:|
//! | getzmqnotifications                | version         | TODO                                   |
//!
//! </details>

#[doc(inline)]
pub use crate::{
    v17::{
        AbortRescan, AddedNode, AddedNodeAddress, AddressInformation, AddressPurpose,
        Bip125Replaceable, Bip32DerivError, BlockTemplateTransaction,
        BlockTemplateTransactionError, BumpFee, BumpFeeError, ChainTips, ChainTipsError,
        ChainTipsStatus, CombinePsbt, CombineRawTransaction, ConvertToPsbt, CreateMultisigError,
        CreatePsbt, CreateRawTransaction, DecodeRawTransaction, EncryptWallet, EstimateRawFee,
        EstimateRawFeeError, EstimateSmartFee, FinalizePsbt, FinalizePsbtError, FundRawTransaction,
        FundRawTransactionError, Generate, GenerateToAddress, GetAddedNodeInfo,
        GetAddressInfoEmbeddedError, GetAddressesByLabel, GetBalance, GetBestBlockHash,
        GetBlockCount, GetBlockHash, GetBlockStatsError, GetBlockTemplate, GetBlockTemplateError,
        GetBlockVerboseZero, GetChainTips, GetChainTxStatsError, GetConnectionCount, GetDifficulty,
        GetMemoryInfoStats, GetMempoolInfoError, GetNetTotals, GetNetworkInfoAddress,
        GetNetworkInfoError, GetNetworkInfoNetwork, GetNewAddress, GetRawChangeAddress,
        GetRawMempool, GetRawTransaction, GetRawTransactionVerbose, GetRawTransactionVerboseError,
        GetReceivedByAddress, GetTransactionDetailError, GetTxOut, GetTxOutError,
        ListAddressGroupings, ListAddressGroupingsError, ListAddressGroupingsItem, ListLabels,
        ListLockUnspent, ListLockUnspentItem, ListLockUnspentItemError, ListReceivedByAddressError,
        ListUnspentItemError, ListWallets, LockUnspent, Locked, NumericError,
        PartialSignatureError, PruneBlockchain, RawFeeDetail, RawFeeRange, RawTransactionError,
        RawTransactionInput, RawTransactionOutput, RescanBlockchain, ScanTxOutSetAbort,
        ScanTxOutSetError, ScanTxOutSetStatus, ScriptType, SendRawTransaction, SendToAddress,
        SetNetworkActive, SignFail, SignFailError, SignMessage, SignMessageWithPrivKey,
        SignRawTransaction, SignRawTransactionError, SignRawTransactionWithKey,
        SignRawTransactionWithWallet, TransactionCategory, UploadTarget, ValidateAddress,
        ValidateAddressError, VerifyChain, VerifyMessage, VerifyTxOutProof, WaitForBlock,
        WaitForBlockError, WaitForBlockHeight, WaitForBlockHeightError, WaitForNewBlock,
        WaitForNewBlockError, WalletCreateFundedPsbt, WalletCreateFundedPsbtError, WitnessUtxo,
        WitnessUtxoError,
    },
    v18::{
        ActiveCommand, AnalyzePsbt, AnalyzePsbtError, AnalyzePsbtInput, AnalyzePsbtInputMissing,
        AnalyzePsbtInputMissingError, DeriveAddresses, GetAddressInfoError, GetReceivedByLabel,
        GetZmqNotifications, JoinPsbts, JsonRpcError, ListReceivedByAddress,
        ListReceivedByAddressItem, ListReceivedByLabel, ListReceivedByLabelError,
        ListReceivedByLabelItem, UtxoUpdatePsbt,
    },
    v19::{
        Bip9SoftforkInfo, Bip9SoftforkStatistics, Bip9SoftforkStatus, GetBalancesMine,
        GetBalancesWatchOnly, GetBlockFilter, GetBlockFilterError, GetChainTxStats, GetRpcInfo,
        MapMempoolEntryError, MempoolEntryError, MempoolEntryFees, MempoolEntryFeesError,
        SetWalletFlag, Softfork, SoftforkType,
    },
    v20::GenerateToDescriptor,
    v21::{
        AddPeerAddress, GetIndexInfo, GetIndexInfoName, GetRawMempoolSequence, ImportDescriptors,
        ImportDescriptorsResult, PsbtBumpFee, PsbtBumpFeeError, Send, SendError, SendMany,
        SendManyVerbose,
    },
    v22::{
        AddConnection, Banned, EnumerateSigners, GetNodeAddresses, ListBanned, NodeAddress,
        ScriptPubKey, Signers, WalletDisplayAddress,
    },
    v23::{
        Bip9Info, Bip9Statistics, CreateMultisig, DecodeScript, DecodeScriptError,
        DecodeScriptSegwit, DeploymentInfo, GetDeploymentInfo, GetDeploymentInfoError,
        RestoreWallet, SaveMempool,
    },
    v24::{
        GetMempoolAncestors, GetMempoolAncestorsVerbose, GetMempoolDescendants,
        GetMempoolDescendantsVerbose, GetMempoolEntry, GetRawMempoolVerbose, GetTransactionDetail,
        GetTxSpendingPrevout, GetTxSpendingPrevoutError, GetTxSpendingPrevoutItem, ListUnspent,
        ListUnspentItem, MempoolEntry, MigrateWallet, SendAll, SendAllError,
        SimulateRawTransaction,
    },
    v25::{
        DescriptorInfo, GenerateBlock, GenerateBlockError, GetBlockStats, ListDescriptors,
        MempoolAcceptanceError, ScanBlocksAbort, ScanBlocksStartError, ScanBlocksStatus,
        TestMempoolAcceptError,
    },
    v26::{
        AddrManInfoNetwork, CreateWallet, DescriptorProcessPsbt, DescriptorProcessPsbtError,
        DumpTxOutSet, DumpTxOutSetError, GetAddrManInfo, GetBalances, GetBalancesError,
        GetPeerInfo, GetTransactionError, GetTxOutSetInfo, GetTxOutSetInfoBlockInfo,
        GetTxOutSetInfoError, GetTxOutSetInfoUnspendables, LoadTxOutSet, LoadTxOutSetError,
        LoadWallet, PeerInfo, ScanBlocksStart, UnloadWallet, WalletProcessPsbt,
        WalletProcessPsbtError,
    },
    v27::{GetPrioritisedTransactions, PrioritisedTransaction},
    v28::{
        CreateWalletDescriptor, GetAddressInfo, GetAddressInfoEmbedded, GetHdKeys, GetHdKeysError,
        GetNetworkInfo, GetRawAddrMan, GetTransaction, HdKey, HdKeyDescriptor, ListSinceBlock,
        ListSinceBlockError, ListTransactions, Logging, RawAddrManEntry, ScanTxOutSetStart,
        ScanTxOutSetUnspent, SubmitPackage, SubmitPackageError, SubmitPackageTxResult,
        SubmitPackageTxResultError, SubmitPackageTxResultFees, SubmitPackageTxResultFeesError,
        TransactionItem, TransactionItemError,
    },
    v29::{
        ActivityEntry, ChainState, DeriveAddressesMultipath, GetBlockHeader, GetBlockHeaderError,
        GetBlockHeaderVerbose, GetBlockHeaderVerboseError, GetBlockVerboseOne,
        GetBlockVerboseOneError, GetBlockVerboseThree, GetBlockVerboseThreeError,
        GetBlockVerboseThreePrevout, GetBlockVerboseThreeTransaction, GetBlockVerboseTwo,
        GetBlockVerboseTwoError, GetBlockVerboseTwoTransaction, GetBlockchainInfo,
        GetBlockchainInfoError, GetChainStates, GetChainStatesError, GetDescriptorActivity,
        GetDescriptorActivityError, GetDescriptorInfo, GetOrphanTxsError,
        GetOrphanTxsVerboseOneEntryError, GetOrphanTxsVerboseTwoEntryError,
        GetRawTransactionVerboseWithPrevout, MempoolAcceptance, MempoolAcceptanceFees,
        NextBlockInfo, NextBlockInfoError, RawTransactionInputWithPrevout, ReceiveActivity,
        SpendActivity, TestMempoolAccept,
    },
    v30::{
        ControlBlocksError, DecodePsbt, DecodePsbtError, GetMempoolInfo, GetMiningInfo,
        GetMiningInfoError, GetOrphanTxs, GetOrphanTxsVerboseOne, GetOrphanTxsVerboseOneEntry,
        GetOrphanTxsVerboseTwo, GetOrphanTxsVerboseTwoEntry, GetWalletInfo, GetWalletInfoError,
        GetWalletInfoScanning, GlobalXpub, GlobalXpubError, LastProcessedBlock,
        LastProcessedBlockError, ListWalletDir, ListWalletDirWallet, Musig2PartialSig,
        Musig2ParticipantPubKeys, Musig2Pubnonce, Proprietary, PsbtInput, PsbtInputError,
        PsbtOutput, PsbtOutputError, TaprootBip32Deriv, TaprootBip32DerivsError, TaprootLeaf,
        TaprootLeafError, TaprootScript, TaprootScriptError, TaprootScriptPathSig,
        TaprootScriptPathSigError,
    },
};
