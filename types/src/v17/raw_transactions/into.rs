// SPDX-License-Identifier: CC0-1.0

use std::collections::BTreeMap;

use bitcoin::psbt::{self, Psbt, PsbtParseError, PsbtSighashType};
use bitcoin::{
    absolute, consensus, hex, transaction, Address, Amount, BlockHash, ScriptBuf, Sequence,
    Transaction, Txid,
};

use super::{
    CombinePsbt, CombineRawTransaction, ConvertToPsbt, CreatePsbt, CreateRawTransaction,
    DecodePsbt, DecodePsbtError, DecodeRawTransaction, DecodeScript, DecodeScriptError,
    DecodeScriptSegwit, DecodeScriptSegwitError, FinalizePsbt, FinalizePsbtError,
    FundRawTransaction, FundRawTransactionError, GetRawTransaction, GetRawTransactionVerbose,
    GetRawTransactionVerboseError, MempoolAcceptance, PsbtInput, PsbtInputError, PsbtOutput,
    PsbtOutputError, SendRawTransaction, SignFail, SignFailError, SignRawTransaction,
    SignRawTransactionError, TestMempoolAccept,
};
use crate::model;
use crate::psbt::RawTransactionError;

impl CombinePsbt {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::CombinePsbt, PsbtParseError> {
        let psbt = self.0.parse::<Psbt>()?;
        Ok(model::CombinePsbt(psbt))
    }

    /// Converts json straight to a `bitcoin::Psbt`.
    pub fn psbt(self) -> Result<Psbt, PsbtParseError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

impl CombineRawTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(
        self,
    ) -> Result<model::CombineRawTransaction, consensus::encode::FromHexError> {
        let tx: Transaction = consensus::encode::deserialize_hex(&self.0)?;
        Ok(model::CombineRawTransaction(tx))
    }

    /// Converts json straight to a `bitcoin::Transaction`.
    pub fn transaction(self) -> Result<Transaction, consensus::encode::FromHexError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

impl ConvertToPsbt {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ConvertToPsbt, PsbtParseError> {
        let psbt = self.0.parse::<Psbt>()?;
        Ok(model::ConvertToPsbt(psbt))
    }

    /// Converts json straight to a `bitcoin::Psbt`.
    pub fn psbt(self) -> Result<Psbt, PsbtParseError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

impl CreatePsbt {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::CreatePsbt, PsbtParseError> {
        let psbt = self.0.parse::<Psbt>()?;
        Ok(model::CreatePsbt(psbt))
    }

    /// Converts json straight to a `bitcoin::Psbt`.
    pub fn psbt(self) -> Result<Psbt, PsbtParseError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

impl CreateRawTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(
        self,
    ) -> Result<model::CreateRawTransaction, consensus::encode::FromHexError> {
        let tx: Transaction = consensus::encode::deserialize_hex(&self.0)?;
        Ok(model::CreateRawTransaction(tx))
    }

    /// Converts json straight to a `bitcoin::Transaction`.
    pub fn transaction(self) -> Result<Transaction, consensus::encode::FromHexError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

impl DecodePsbt {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::DecodePsbt, DecodePsbtError> {
        use DecodePsbtError as E;

        let unsigned_tx = self.tx.to_transaction().map_err(E::Tx)?;
        let unknown = match self.unknown {
            Some(map) => crate::psbt::into_unknown(map).map_err(E::Unknown)?,
            None => BTreeMap::default(),
        };

        let inputs = self
            .inputs
            .into_iter()
            .map(|input| input.into_input())
            .collect::<Result<_, _>>()
            .map_err(E::Inputs)?;
        let outputs = self
            .outputs
            .into_iter()
            .map(|output| output.into_output())
            .collect::<Result<_, _>>()
            .map_err(E::Outputs)?;

        // These fields do not appear until Core v23.
        let version = 0;
        let xpub = BTreeMap::default();
        let proprietary = BTreeMap::default();

        let psbt =
            bitcoin::Psbt { unsigned_tx, version, xpub, proprietary, unknown, inputs, outputs };
        let fee = self.fee.map(Amount::from_sat);

        Ok(model::DecodePsbt { psbt, fee })
    }
}

impl PsbtInput {
    /// Converts this PSBT data into a PSBT input.
    pub fn into_input(self) -> Result<psbt::Input, PsbtInputError> {
        use PsbtInputError as E;

        let non_witness_utxo = self
            .non_witness_utxo
            .map(|raw| raw.to_transaction())
            .transpose()
            .map_err(E::NonWitnessUtxo)?;
        let witness_utxo =
            self.witness_utxo.map(|utxo| utxo.to_tx_out()).transpose().map_err(E::WitnessUtxo)?;
        let partial_sigs = match self.partial_signatures {
            Some(map) => crate::psbt::into_partial_signatures(map).map_err(E::PartialSignatures)?,
            None => BTreeMap::default(),
        };
        let sighash_type = self
            .sighash
            .map(|partial| partial.parse::<PsbtSighashType>())
            .transpose()
            .map_err(E::Sighash)?;
        let redeem_script = self
            .redeem_script
            .map(|script| script.script_buf())
            .transpose()
            .map_err(E::RedeemScript)?;
        let witness_script = self
            .witness_script
            .map(|script| script.script_buf())
            .transpose()
            .map_err(E::WitnessScript)?;
        let bip32_derivation = match self.bip32_derivs {
            Some(derivs) =>
                crate::psbt::map_into_bip32_derivation(derivs).map_err(E::Bip32Derivs)?,
            None => BTreeMap::default(),
        };
        let final_script_sig = self
            .final_script_sig
            .map(|script| script.script_buf())
            .transpose()
            .map_err(E::FinalScriptSig)?;
        let final_script_witness = self
            .final_script_witness
            .map(|v| crate::witness_from_hex_slice(&v))
            .transpose()
            .map_err(E::FinalScriptWitness)?;
        let unknown = match self.unknown {
            Some(map) => crate::psbt::into_unknown(map).map_err(E::Unknown)?,
            None => BTreeMap::default(),
        };

        // These fields do not appear until Core v23.
        let ripemd160_preimages = BTreeMap::default();
        let sha256_preimages = BTreeMap::default();
        let hash160_preimages = BTreeMap::default();
        let hash256_preimages = BTreeMap::default();
        let proprietary = BTreeMap::default();

        // These fields do not appear until Core v24.
        let tap_key_sig = None;
        let tap_script_sigs = BTreeMap::default();
        let tap_scripts = BTreeMap::default();
        let tap_key_origins = BTreeMap::default();
        let tap_internal_key = None;
        let tap_merkle_root = None;

        Ok(psbt::Input {
            non_witness_utxo,
            witness_utxo,
            partial_sigs,
            sighash_type,
            redeem_script,
            witness_script,
            bip32_derivation,
            final_script_sig,
            final_script_witness,
            ripemd160_preimages,
            sha256_preimages,
            hash160_preimages,
            hash256_preimages,
            tap_key_sig,
            tap_script_sigs,
            tap_scripts,
            tap_key_origins,
            tap_internal_key,
            tap_merkle_root,
            proprietary,
            unknown,
        })
    }
}

impl PsbtOutput {
    /// Converts this PSBT data into a PSBT output.
    pub fn into_output(self) -> Result<psbt::Output, PsbtOutputError> {
        use PsbtOutputError as E;

        let redeem_script = self
            .redeem_script
            .map(|script| script.script_buf())
            .transpose()
            .map_err(E::RedeemScript)?;
        let witness_script = self
            .witness_script
            .map(|script| script.script_buf())
            .transpose()
            .map_err(E::WitnessScript)?;
        let bip32_derivation = match self.bip32_derivs {
            Some(derivs) =>
                crate::psbt::vec_into_bip32_derivation(derivs).map_err(E::Bip32Derivs)?,
            None => BTreeMap::default(),
        };
        let unknown = match self.unknown {
            Some(map) => crate::psbt::into_unknown(map).map_err(E::Unknown)?,
            None => BTreeMap::default(),
        };

        // This field does not appear until Core v23.
        let proprietary = BTreeMap::default();

        // These fields do not appear until Core v24.
        let tap_internal_key = None;
        let tap_tree = None;
        let tap_key_origins = BTreeMap::default();

        Ok(psbt::Output {
            redeem_script,
            witness_script,
            bip32_derivation,
            tap_internal_key,
            tap_tree,
            tap_key_origins,
            proprietary,
            unknown,
        })
    }
}

impl DecodeRawTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::DecodeRawTransaction, RawTransactionError> {
        let raw_tx = self.0.to_transaction()?;
        Ok(model::DecodeRawTransaction(raw_tx))
    }

    /// Converts json straight to a `bitcoin::Transaction`.
    pub fn transaction(self) -> Result<Transaction, RawTransactionError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

impl DecodeScript {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::DecodeScript, DecodeScriptError> {
        use DecodeScriptError as E;

        let script_pubkey = match self.hex {
            Some(hex) => Some(ScriptBuf::from_hex(&hex).map_err(E::Hex)?),
            None => None,
        };
        let addresses = match self.addresses {
            Some(addresses) => addresses
                .iter()
                .map(|s| s.parse::<Address<_>>())
                .collect::<Result<_, _>>()
                .map_err(E::Addresses)?,
            None => vec![],
        };
        let p2sh = self.p2sh.map(|s| s.parse::<Address<_>>()).transpose().map_err(E::P2sh)?;

        Ok(model::DecodeScript {
            script_pubkey,
            type_: self.type_,
            descriptor: None,
            address: None,
            required_signatures: self.required_signatures,
            addresses,
            p2sh,
            segwit: None,
        })
    }
}
#[allow(dead_code)]
impl DecodeScriptSegwit {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::DecodeScriptSegwit, DecodeScriptSegwitError> {
        use DecodeScriptSegwitError as E;

        // Convert `Option<Vec<String>>` to `Vec<Address<NetworkUnchecked>>`
        let addresses = match self.addresses {
            Some(addrs) => addrs
                .into_iter()
                .map(|s| s.parse::<Address<_>>())
                .collect::<Result<_, _>>()
                .map_err(E::Addresses)?,
            None => vec![],
        };

        let required_signatures = self.required_signatures;
        let p2sh_segwit = self.p2sh_segwit;

        Ok(model::DecodeScriptSegwit {
            asm: self.asm,
            hex: self.hex,
            descriptor: None,
            address: None,
            type_: self.type_,
            required_signatures,
            addresses,
            p2sh_segwit,
        })
    }
}

impl FinalizePsbt {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::FinalizePsbt, FinalizePsbtError> {
        use FinalizePsbtError as E;

        let psbt = self.psbt.map(|s| s.parse::<Psbt>()).transpose().map_err(E::Psbt)?;
        let tx = match self.hex {
            Some(hex) => Some(consensus::encode::deserialize_hex(&hex).map_err(E::Hex)?),
            None => None,
        };

        Ok(model::FinalizePsbt { psbt, complete: self.complete, tx })
    }
}

impl FundRawTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::FundRawTransaction, FundRawTransactionError> {
        use FundRawTransactionError as E;

        let tx: Transaction = consensus::encode::deserialize_hex(&self.hex).map_err(E::Hex)?;
        let fee = Amount::from_btc(self.fee).map_err(E::Fee)?;

        Ok(model::FundRawTransaction { tx, fee, change_position: self.change_position })
    }

    /// Converts json straight to a `bitcoin::Transaction`.
    pub fn transaction(self) -> Result<Transaction, FundRawTransactionError> {
        let model = self.into_model()?;
        Ok(model.tx)
    }
}

impl GetRawTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetRawTransaction, consensus::encode::FromHexError> {
        let tx: Transaction = consensus::encode::deserialize_hex(&self.0)?;
        Ok(model::GetRawTransaction(tx))
    }

    /// Converts json straight to a `bitcoin::Transaction`.
    pub fn transaction(self) -> Result<bitcoin::Transaction, consensus::encode::FromHexError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

impl GetRawTransactionVerbose {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(
        self,
    ) -> Result<model::GetRawTransactionVerbose, GetRawTransactionVerboseError> {
        use GetRawTransactionVerboseError as E;

        let version = transaction::Version::non_standard(self.version);
        let lock_time = absolute::LockTime::from_consensus(self.lock_time);

        let input = self
            .inputs
            .into_iter()
            .map(|input| input.to_input())
            .collect::<Result<_, _>>()
            .map_err(E::Inputs)?;
        let output = self
            .outputs
            .into_iter()
            .map(|output| output.to_output())
            .collect::<Result<_, _>>()
            .map_err(E::Outputs)?;

        let transaction = Transaction { version, lock_time, input, output };
        let block_hash =
            self.block_hash.map(|s| s.parse::<BlockHash>()).transpose().map_err(E::BlockHash)?;

        Ok(model::GetRawTransactionVerbose {
            in_active_chain: self.in_active_chain,
            transaction,
            block_hash,
            confirmations: self.confirmations,
            transaction_time: self.transaction_time,
            block_time: self.block_time,
        })
    }
}

impl SendRawTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::SendRawTransaction, hex::HexToArrayError> {
        let txid = self.0.parse::<Txid>()?;
        Ok(model::SendRawTransaction(txid))
    }

    /// Converts json straight to a `bitcoin::Txid`.
    pub fn txid(self) -> Result<Txid, hex::HexToArrayError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

impl SignRawTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::SignRawTransaction, SignRawTransactionError> {
        use SignRawTransactionError as E;

        let tx: Transaction = consensus::encode::deserialize_hex(&self.hex).map_err(E::Hex)?;

        let errors = match self.errors {
            Some(v) => v
                .into_iter()
                .map(|f| f.into_model())
                .collect::<Result<_, _>>()
                .map_err(E::Errors)?,
            None => vec![],
        };

        Ok(model::SignRawTransaction { tx, complete: self.complete, errors })
    }
}

impl SignFail {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::SignFail, SignFailError> {
        use SignFailError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let script_sig = ScriptBuf::from_hex(&self.script_sig).map_err(E::ScriptSig)?;
        let sequence = Sequence::from_consensus(self.sequence);

        Ok(model::SignFail { txid, vout: self.vout, script_sig, sequence, error: self.error })
    }
}

impl TestMempoolAccept {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::TestMempoolAccept, hex::HexToArrayError> {
        let results = self.0.into_iter().map(|r| r.into_model()).collect::<Result<_, _>>()?;

        Ok(model::TestMempoolAccept { results })
    }
}

impl MempoolAcceptance {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::MempoolAcceptance, hex::HexToArrayError> {
        let txid = self.txid.parse::<Txid>()?;

        Ok(model::MempoolAcceptance {
            txid,
            wtxid: None, // v22 and later only.
            allowed: self.allowed,
            vsize: None, // v21 and later only.
            fees: None,  // v21 and later only.
            reject_reason: self.reject_reason,
            reject_details: None, // v29 and later only.
        })
    }
}
