// SPDX-License-Identifier: CC0-1.0

//! Models of the data returned by the JSON-RPC API of Bitcoin Core.
//!
//! The types here model the data returned by Bitcoin Core in a version nonspecific way. In other
//! words one can use a particular `bitcoind` version via the version specific module (e.g.
//! `crate::v26`) then convert the `json` types to one of the modelled types in this module using
//! `TryFrom`.

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
        Bip9SoftforkInfo, Bip9SoftforkStatistics, Bip9SoftforkStatus, ChainTips, ChainTipsStatus,
        GetBestBlockHash, GetBlockCount, GetBlockHash, GetBlockHeader, GetBlockHeaderVerbose,
        GetBlockStats, GetBlockVerbosityOne, GetBlockVerbosityZero, GetBlockchainInfo,
        GetChainTips, GetChainTxStats, GetDifficulty, GetMempoolAncestors,
        GetMempoolAncestorsVerbose, GetMempoolDescendants, GetMempoolDescendantsVerbose,
        GetMempoolEntry, GetMempoolInfo, GetRawMempool, GetRawMempoolVerbose, GetTxOut,
        GetTxOutProof, GetTxOutSetInfo, MempoolEntry, MempoolEntryFees, Softfork, SoftforkType,
        VerifyTxOutProof,
    },
    generating::{Generate, GenerateToAddress},
    network::{GetNetworkInfo, GetNetworkInfoAddress, GetNetworkInfoNetwork},
    raw_transactions::{
        SendRawTransaction, SubmitPackage, SubmitPackageTxResult, SubmitPackageTxResultFees,
    },
    wallet::{
        AddMultisigAddress, AddressInformation, AddressLabel, AddressPurpose, Bip125Replaceable,
        BumpFee, CreateWallet, DumpPrivKey, DumpWallet, GetAddressInfo, GetAddressInfoEmbedded,
        GetAddressesByLabel, GetBalance, GetBalances, GetBalancesMine, GetBalancesWatchOnly,
        GetNewAddress, GetRawChangeAddress, GetReceivedByAddress, GetTransaction,
        GetTransactionDetail, GetUnconfirmedBalance, GetWalletInfo, ListAddressGroupings,
        ListAddressGroupingsItem, ListLabels, ListLockUnspent, ListLockUnspentItem,
        ListReceivedByAddress, ListReceivedByAddressItem, ListSinceBlock,
        ListSinceBlockTransaction, ListTransactions, ListTransactionsItem, ListUnspentItem,
        ListWallets, LoadWallet, RescanBlockchain, ScriptType, SendMany, SendToAddress,
        SignErrorData, SignMessage, SignRawTransactionWithWallet, TransactionCategory,
        UnloadWallet, WalletCreateFundedPsbt, WalletProcessPsbt,
    },
};
