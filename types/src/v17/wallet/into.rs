// SPDX-License-Identifier: CC0-1.0

use bitcoin::address::NetworkUnchecked;
use bitcoin::amount::ParseAmountError;
use bitcoin::consensus::encode;
use bitcoin::hashes::hash160;
use bitcoin::hex::FromHex;
use bitcoin::key::{self, PrivateKey, PublicKey};
use bitcoin::psbt::PsbtParseError;
use bitcoin::{
    address, bip32, sign_message, Address, Amount, BlockHash, Psbt, ScriptBuf, SignedAmount,
    Transaction, Txid, WitnessProgram, WitnessVersion,
};

// TODO: Use explicit imports?
use super::*;
use crate::{model, NumericError};

impl AddressPurpose {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::AddressPurpose {
        use AddressPurpose::*;

        match self {
            Send => model::AddressPurpose::Send,
            Receive => model::AddressPurpose::Receive,
        }
    }
}

impl TransactionCategory {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::TransactionCategory {
        use model::TransactionCategory as M; // M for model.
        use TransactionCategory as V; // V for version specific.

        match self {
            V::Send => M::Send,
            V::Receive => M::Receive,
            V::Generate => M::Generate,
            V::Immature => M::Immature,
            V::Orphan => M::Orphan,
        }
    }
}

impl Bip125Replaceable {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::Bip125Replaceable {
        use model::Bip125Replaceable as M; // M for model.
        use Bip125Replaceable as V; // V for version specific.

        match self {
            V::Yes => M::Yes,
            V::No => M::No,
            V::Unknown => M::Unknown,
        }
    }
}

impl AddMultisigAddress {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::AddMultisigAddress, AddMultisigAddressError> {
        use AddMultisigAddressError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let redeem_script = ScriptBuf::from_hex(&self.redeem_script).map_err(E::RedeemScript)?;

        Ok(model::AddMultisigAddress { address, redeem_script })
    }
}

impl BumpFee {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::BumpFee, BumpFeeError> {
        use BumpFeeError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let original_fee = Amount::from_btc(self.original_fee).map_err(E::OriginalFee)?;
        let fee = Amount::from_btc(self.fee).map_err(E::Fee)?;

        Ok(model::BumpFee { txid, original_fee, fee, errors: self.errors })
    }
}

impl CreateWallet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::CreateWallet {
        model::CreateWallet { name: self.name, warnings: vec![self.warning] }
    }
}

impl DumpPrivKey {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::DumpPrivKey, key::FromWifError> {
        let key = self.0.parse::<PrivateKey>()?;
        Ok(model::DumpPrivKey(key))
    }
}

impl AddressInformation {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::AddressInformation {
        model::AddressInformation { purpose: self.purpose.into_model() }
    }
}

impl GetAddressesByLabel {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetAddressesByLabel, address::ParseError> {
        let mut map = BTreeMap::new();

        for (k, v) in self.0.into_iter() {
            let address = k.parse::<Address<_>>()?;
            let info = v.into_model();
            map.insert(address, info);
        }

        Ok(model::GetAddressesByLabel(map))
    }
}

impl GetAddressInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetAddressInfo, GetAddressInfoError> {
        use GetAddressInfoError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let script_pubkey = ScriptBuf::from_hex(&self.script_pubkey).map_err(E::ScriptPubkey)?;
        let (witness_version, witness_program) = match (self.witness_version, self.witness_program)
        {
            (Some(v), Some(hex)) => {
                if v > u8::MAX as i64 || v < 0 {
                    return Err(E::WitnessVersionValue(v));
                }
                let witness_version =
                    WitnessVersion::try_from(v as u8).map_err(E::WitnessVersion)?;

                let bytes = Vec::from_hex(&hex).map_err(E::WitnessProgramBytes)?;
                let witness_program =
                    WitnessProgram::new(witness_version, &bytes).map_err(E::WitnessProgram)?;

                (Some(witness_version), Some(witness_program))
            }
            _ => (None, None), // TODO: Think more if catchall is ok.
        };
        let script = self.script.map(|s| s.into_model());
        let redeem_script =
            self.hex.map(|hex| ScriptBuf::from_hex(&hex).map_err(E::Hex)).transpose()?;
        let pubkeys = self
            .pubkeys
            .map(|pubkeys| {
                pubkeys
                    .iter()
                    .map(|s| s.parse::<PublicKey>())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(E::Pubkeys)
            })
            .transpose()?;
        let sigs_required =
            self.sigs_required.map(|s| crate::to_u32(s, "sigs_required")).transpose()?;
        let pubkey = self.pubkey.map(|s| s.parse::<PublicKey>()).transpose().map_err(E::Pubkey)?;
        let embedded =
            self.embedded.map(|embedded| embedded.into_model()).transpose().map_err(E::Embedded)?;
        let hd_key_path = self
            .hd_key_path
            .map(|s| s.parse::<bip32::DerivationPath>())
            .transpose()
            .map_err(E::HdKeyPath)?;
        let hd_seed_id =
            self.hd_seed_id.map(|s| s.parse::<hash160::Hash>()).transpose().map_err(E::HdSeedId)?;
        let labels = self.labels.into_iter().map(|label| label.into_model()).collect();

        Ok(model::GetAddressInfo {
            address,
            script_pubkey,
            is_mine: self.is_mine,
            is_watch_only: self.is_watch_only,
            is_script: self.is_script,
            is_witness: self.is_witness,
            witness_version,
            witness_program,
            script,
            hex: redeem_script,
            pubkeys,
            sigs_required,
            pubkey,
            embedded,
            is_compressed: self.is_compressed,
            label: self.label,
            timestamp: self.timestamp,
            hd_key_path,
            hd_seed_id,
            labels,
        })
    }
}

impl ScriptType {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::ScriptType {
        use model::ScriptType as M; // M for model.
        use ScriptType as V; // V for version specific.

        match self {
            V::NonStandard => M::NonStandard,
            V::Pubkey => M::Pubkey,
            V::PubkeyHash => M::PubkeyHash,
            V::ScriptHash => M::ScriptHash,
            V::Multisig => M::Multisig,
            V::NullData => M::NullData,
            V::WitnessV0KeyHash => M::WitnessV0KeyHash,
            V::WitnessV0ScriptHash => M::WitnessV0ScriptHash,
            V::WitnessUnknown => M::WitnessUnknown,
        }
    }
}

impl GetAddressInfoEmbedded {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetAddressInfoEmbedded, GetAddressInfoEmbeddedError> {
        use GetAddressInfoEmbeddedError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let script_pubkey = ScriptBuf::from_hex(&self.script_pubkey).map_err(E::ScriptPubkey)?;
        let (witness_version, witness_program) = match (self.witness_version, self.witness_program)
        {
            (Some(v), Some(hex)) => {
                if v > u8::MAX as i64 || v < 0 {
                    return Err(E::WitnessVersionValue(v));
                }
                let witness_version =
                    WitnessVersion::try_from(v as u8).map_err(E::WitnessVersion)?;

                let bytes = Vec::from_hex(&hex).map_err(E::WitnessProgramBytes)?;
                let witness_program =
                    WitnessProgram::new(witness_version, &bytes).map_err(E::WitnessProgram)?;

                (Some(witness_version), Some(witness_program))
            }
            _ => (None, None), // TODO: Think more if catchall is ok.
        };
        let script = self.script.map(|s| s.into_model());
        let redeem_script =
            self.hex.map(|hex| ScriptBuf::from_hex(&hex).map_err(E::Hex)).transpose()?;
        let pubkeys = self
            .pubkeys
            .iter()
            .map(|s| s.parse::<PublicKey>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::Pubkeys)?;
        let sigs_required =
            self.sigs_required.map(|s| crate::to_u32(s, "sigs_required")).transpose()?;
        let pubkey = self.pubkey.map(|s| s.parse::<PublicKey>()).transpose().map_err(E::Pubkey)?;
        let labels = self.labels.into_iter().map(|label| label.into_model()).collect();

        Ok(model::GetAddressInfoEmbedded {
            address,
            script_pubkey,
            is_script: self.is_script,
            is_witness: self.is_witness,
            witness_version,
            witness_program,
            script,
            hex: redeem_script,
            pubkeys,
            sigs_required,
            pubkey,
            is_compressed: self.is_compressed,
            label: self.label,
            labels,
        })
    }
}

impl GetAddressInfoLabel {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::AddressLabel {
        model::AddressLabel { name: self.name, purpose: self.purpose.into_model() }
    }
}

impl GetBalance {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBalance, ParseAmountError> {
        let amount = Amount::from_btc(self.0)?;
        Ok(model::GetBalance(amount))
    }
}

impl GetNewAddress {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetNewAddress, address::ParseError> {
        let address = self.0.parse::<Address<_>>()?;
        Ok(model::GetNewAddress(address))
    }

    /// Converts json straight to a `bitcoin::Address`.
    pub fn address(self) -> Result<Address<NetworkUnchecked>, address::ParseError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

impl GetRawChangeAddress {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetRawChangeAddress, address::ParseError> {
        let address = self.0.parse::<Address<_>>()?;
        Ok(model::GetRawChangeAddress(address))
    }

    /// Converts json straight to a `bitcoin::Address`.
    pub fn address(self) -> Result<Address<NetworkUnchecked>, address::ParseError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

impl GetReceivedByAddress {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetReceivedByAddress, ParseAmountError> {
        let amount = Amount::from_btc(self.0)?;
        Ok(model::GetReceivedByAddress(amount))
    }
}

impl GetTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTransaction, GetTransactionError> {
        use GetTransactionError as E;

        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        let fee = self.fee.map(|fee| SignedAmount::from_btc(fee).map_err(E::Fee)).transpose()?;
        let block_hash =
            self.block_hash.map(|s| s.parse::<BlockHash>().map_err(E::BlockHash)).transpose()?;
        let block_index =
            self.block_index.map(|idx| crate::to_u32(idx, "block_index")).transpose()?;
        let block_height =
            self.block_height.map(|h| crate::to_u32(h, "block_height")).transpose()?;
        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let wtxid = self.wtxid.map(|s| s.parse::<Txid>().map_err(E::Wtxid)).transpose()?;
        let wallet_conflicts = self
            .wallet_conflicts
            .into_iter()
            .map(|s| s.parse::<Txid>().map_err(E::WalletConflicts))
            .collect::<Result<Vec<_>, _>>()?;
        let replaced_by_txid = self
            .replaced_by_txid
            .map(|s| s.parse::<Txid>().map_err(E::ReplacedByTxid))
            .transpose()?;
        let replaces_txid =
            self.replaces_txid.map(|s| s.parse::<Txid>().map_err(E::ReplacesTxid)).transpose()?;
        let mempool_conflicts = self
            .mempool_conflicts
            .map(|vec| {
                vec.into_iter()
                    .map(|s| s.parse::<Txid>().map_err(E::MempoolConflicts))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let tx = encode::deserialize_hex::<Transaction>(&self.hex).map_err(E::Tx)?;
        let details = self
            .details
            .into_iter()
            .map(|d| d.into_model().map_err(E::Details))
            .collect::<Result<Vec<_>, _>>()?;
        let last_processed_block = self
            .last_processed_block
            .map(|l| l.into_model())
            .transpose()
            .map_err(E::LastProcessedBlock)?;

        Ok(model::GetTransaction {
            amount,
            fee,
            confirmations: self.confirmations,
            generated: self.generated,
            trusted: self.trusted,
            block_hash,
            block_height,
            block_index,
            block_time: self.block_time,
            txid,
            wtxid,
            wallet_conflicts,
            replaced_by_txid,
            replaces_txid,
            mempool_conflicts,
            to: self.to,
            time: self.time,
            time_received: self.time_received,
            comment: self.comment,
            bip125_replaceable: self.bip125_replaceable.into_model(),
            parent_descriptors: self.parent_descriptors,
            details,
            hex: Some(self.hex),
            decoded: self.decoded,
            last_processed_block,
            tx,
        })
    }
}

impl GetTransactionDetail {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTransactionDetail, GetTransactionDetailError> {
        use GetTransactionDetailError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        let fee = self.fee.map(|fee| SignedAmount::from_btc(fee).map_err(E::Fee)).transpose()?;

        Ok(model::GetTransactionDetail {
            address,
            category: self.category.into_model(),
            amount,
            label: self.label,
            vout: self.vout,
            fee,
            abandoned: self.abandoned,
        })
    }
}

impl LastProcessedBlock {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::LastProcessedBlock, LastProcessedBlockError> {
        let hash = self.hash.parse::<BlockHash>().map_err(LastProcessedBlockError::Hash)?;
        let height = crate::to_u32(self.height, "height")?;
        Ok(model::LastProcessedBlock { height, hash })
    }
}

impl GetUnconfirmedBalance {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetUnconfirmedBalance, ParseAmountError> {
        let amount = Amount::from_btc(self.0)?;
        Ok(model::GetUnconfirmedBalance(amount))
    }
}

impl GetWalletInfo {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetWalletInfo, GetWalletInfoError> {
        use GetWalletInfoError as E;

        let wallet_version = crate::to_u32(self.wallet_version, "wallet_version")?;
        let balance = Amount::from_btc(self.balance).map_err(E::Balance)?;
        let unconfirmed_balance =
            Amount::from_btc(self.unconfirmed_balance).map_err(E::UnconfirmedBalance)?;
        let immature_balance =
            Amount::from_btc(self.immature_balance).map_err(E::ImmatureBalance)?;
        let tx_count = crate::to_u32(self.tx_count, "tx_count")?;
        let keypool_oldest = crate::to_u32(self.keypool_oldest, "keypoo_oldest")?;
        let keypool_size = crate::to_u32(self.keypool_size, "keypoo_size")?;
        let keypool_size_hd_internal =
            crate::to_u32(self.keypool_size_hd_internal, "keypoo_size_hd_internal")?;
        let pay_tx_fee = crate::btc_per_kb(self.pay_tx_fee).map_err(E::PayTxFee)?;
        let hd_seed_id =
            self.hd_seed_id.map(|s| s.parse::<hash160::Hash>()).transpose().map_err(E::HdSeedId)?;

        Ok(model::GetWalletInfo {
            wallet_name: self.wallet_name,
            wallet_version,
            balance,
            unconfirmed_balance,
            immature_balance,
            tx_count,
            keypool_oldest,
            keypool_size,
            keypool_size_hd_internal,
            unlocked_until: self.unlocked_until,
            pay_tx_fee,
            hd_seed_id,
            private_keys_enabled: self.private_keys_enabled,
        })
    }
}

impl ListAddressGroupings {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListAddressGroupings, ListAddressGroupingsError> {
        todo!() // Don't do this till we work out what the docs mean.
    }
}

impl ListAddressGroupingsItem {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListAddressGroupingsItem, ListAddressGroupingsError> {
        use ListAddressGroupingsError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let amount = Amount::from_btc(self.amount).map_err(E::Amount)?;

        Ok(model::ListAddressGroupingsItem { address, amount, label: self.label })
    }
}

impl ListLabels {
    pub fn into_model(self) -> model::ListLabels { model::ListLabels(self.0) }
}

impl ListLockUnspent {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListLockUnspent, ListLockUnspentItemError> {
        let outputs =
            self.0.into_iter().map(|output| output.into_model()).collect::<Result<Vec<_>, _>>()?;
        Ok(model::ListLockUnspent(outputs))
    }
}

impl ListLockUnspentItem {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListLockUnspentItem, ListLockUnspentItemError> {
        use ListLockUnspentItemError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let vout = crate::to_u32(self.vout, "vout")?;

        Ok(model::ListLockUnspentItem { txid, vout })
    }
}

impl ListReceivedByAddress {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListReceivedByAddress, ListReceivedByAddressError> {
        let balances = self
            .0
            .into_iter()
            .map(|balance| balance.into_model())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(model::ListReceivedByAddress(balances))
    }
}

impl ListReceivedByAddressItem {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(
        self,
    ) -> Result<model::ListReceivedByAddressItem, ListReceivedByAddressError> {
        use ListReceivedByAddressError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let amount = Amount::from_btc(self.amount).map_err(E::Amount)?;
        let txids = self
            .txids
            .iter()
            .enumerate()
            .map(|(i, txid)| {
                txid.parse::<Txid>().map_err(|e| ListReceivedByAddressError::Txids(i, e))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(model::ListReceivedByAddressItem {
            involves_watch_only: self.involves_watch_only,
            address,
            amount,
            confirmations: self.confirmations,
            label: self.label,
            txids,
        })
    }
}

impl ListSinceBlock {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListSinceBlock, ListSinceBlockError> {
        use ListSinceBlockError as E;

        let transactions = self
            .transactions
            .into_iter()
            .map(|tx| tx.into_model())
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::Transactions)?;
        let removed = self
            .removed
            .into_iter()
            .map(|tx| tx.into_model())
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::Removed)?;
        let last_block = self.last_block.parse::<BlockHash>().map_err(E::LastBlock)?;

        Ok(model::ListSinceBlock { transactions, removed, last_block })
    }
}

impl ListSinceBlockTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(
        self,
    ) -> Result<model::ListSinceBlockTransaction, ListSinceBlockTransactionError> {
        use ListSinceBlockTransactionError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let category = self.category.into_model();
        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        let vout = crate::to_u32(self.vout, "vout")?;
        let fee = SignedAmount::from_btc(self.fee).map_err(E::Fee)?;
        let block_hash = self.block_hash.parse::<BlockHash>().map_err(E::BlockHash)?;
        let block_index = crate::to_u32(self.block_index, "block_index")?;
        let txid = self.txid.map(|s| s.parse::<Txid>().map_err(E::Txid)).transpose()?;
        let bip125_replaceable = self.bip125_replaceable.into_model();

        Ok(model::ListSinceBlockTransaction {
            address: Some(address),
            category,
            amount,
            vout,
            fee,
            confirmations: self.confirmations,
            block_hash,
            block_index,
            block_time: self.block_time,
            txid,
            time: self.time,
            time_received: self.time_received,
            bip125_replaceable,
            abandoned: self.abandoned,
            comment: self.comment,
            label: self.label,
            to: self.to,
        })
    }
}

impl ListTransactions {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListTransactions, ListTransactionsItemError> {
        let transactions =
            self.0.into_iter().map(|tx| tx.into_model()).collect::<Result<Vec<_>, _>>()?;
        Ok(model::ListTransactions(transactions))
    }
}

impl ListTransactionsItem {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListTransactionsItem, ListTransactionsItemError> {
        use ListTransactionsItemError as E;

        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let category = self.category.into_model();
        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        let vout = crate::to_u32(self.vout, "vout")?;
        let fee = SignedAmount::from_btc(self.fee).map_err(E::Fee)?;
        let block_hash = self.block_hash.parse::<BlockHash>().map_err(E::BlockHash)?;
        let block_index = crate::to_u32(self.block_index, "block_index")?;
        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let bip125_replaceable = self.bip125_replaceable.into_model();

        Ok(model::ListTransactionsItem {
            address,
            category,
            amount,
            label: self.label,
            vout,
            fee,
            confirmations: self.confirmations,
            trusted: self.trusted,
            block_hash,
            block_index,
            block_time: self.block_time,
            txid,
            time: self.time,
            time_received: self.time_received,
            comment: self.comment,
            bip125_replaceable,
            abandoned: self.abandoned,
        })
    }
}

impl ListUnspent {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListUnspent, ListUnspentItemError> {
        self.0
            .into_iter()
            .map(|item| item.into_model())
            .collect::<Result<Vec<_>, _>>()
            .map(model::ListUnspent)
    }
}

impl ListUnspentItem {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ListUnspentItem, ListUnspentItemError> {
        use ListUnspentItemError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let vout = crate::to_u32(self.vout, "vout")?;
        let address = self.address.parse::<Address<_>>().map_err(E::Address)?;
        let script_pubkey = ScriptBuf::from_hex(&self.script_pubkey).map_err(E::ScriptPubkey)?;

        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        let confirmations = crate::to_u32(self.confirmations, "confirmations")?;
        let redeem_script = self
            .redeem_script
            .map(|hex| ScriptBuf::from_hex(&hex).map_err(E::RedeemScript))
            .transpose()?;

        Ok(model::ListUnspentItem {
            txid,
            vout,
            address,
            label: self.label,
            script_pubkey,
            amount,
            confirmations,
            redeem_script,
            spendable: self.spendable,
            solvable: self.solvable,
            descriptor: None,
            safe: self.safe,
        })
    }
}

impl ListWallets {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::ListWallets { model::ListWallets(self.0) }
}

impl LoadWallet {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::LoadWallet {
        model::LoadWallet { name: self.name, warnings: vec![self.warning] }
    }
}

impl RescanBlockchain {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::RescanBlockchain, NumericError> {
        let start_height = crate::to_u32(self.start_height, "start_height")?;
        let stop_height = crate::to_u32(self.stop_height, "stop_height")?;

        Ok(model::RescanBlockchain { start_height, stop_height })
    }
}

impl SendMany {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::SendMany, hex::HexToArrayError> {
        let txid = self.0.parse::<Txid>()?;
        Ok(model::SendMany(txid))
    }
}

impl SendToAddress {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::SendToAddress, hex::HexToArrayError> {
        let txid = self.0.parse::<Txid>()?;
        Ok(model::SendToAddress { txid })
    }
}

impl SignMessage {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::SignMessage, sign_message::MessageSignatureError> {
        let sig = self.0.parse::<sign_message::MessageSignature>()?;
        Ok(model::SignMessage(sig))
    }
}

impl WalletCreateFundedPsbt {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::WalletCreateFundedPsbt, WalletCreateFundedPsbtError> {
        use WalletCreateFundedPsbtError as E;

        let psbt = self.psbt.parse::<Psbt>().map_err(E::Psbt)?;
        let fee = SignedAmount::from_btc(self.fee).map_err(E::Fee)?;
        let change_pos = crate::to_u32(self.change_pos, "change_pos")?;
        Ok(model::WalletCreateFundedPsbt { psbt, fee, change_pos })
    }
}

impl WalletProcessPsbt {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::WalletProcessPsbt, PsbtParseError> {
        let psbt = self.psbt.parse::<Psbt>()?;
        Ok(model::WalletProcessPsbt { psbt, complete: self.complete })
    }
}
