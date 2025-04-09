// SPDX-License-Identifier: CC0-1.0

use bitcoin::psbt::{Psbt, PsbtParseError};
use bitcoin::{
    absolute, consensus, hex, transaction, Address, Amount, BlockHash, OutPoint, ScriptBuf,
    Sequence, Transaction, TxIn, TxOut, Txid, Witness,
};

use super::{
    CombinePsbt, CombineRawTransaction, ConvertToPsbt, CreatePsbt, CreateRawTransaction,
    DecodePsbt, DecodeRawTransaction, DecodeScript, DecodeScriptError, FinalizePsbt,
    FundRawTransaction, FundRawTransactionError, GetRawTransaction, GetRawTransactionVerbose,
    GetRawTransactionVerboseError, MempoolAcceptance, RawTransaction, RawTransactionError,
    RawTransactionInput, RawTransactionInputError, RawTransactionOutput, RawTransactionOutputError,
    SendRawTransaction, SignFail, SignFailError, SignRawTransaction, SignRawTransactionError,
    TestMempoolAccept,
};
use crate::model;

fn convert_transaction(json: RawTransaction) -> Result<bitcoin::Transaction, RawTransactionError> {
    use RawTransactionError as E;

    let version = transaction::Version::non_standard(json.version);
    let lock_time = absolute::LockTime::from_consensus(json.lock_time);
    let input = json
        .inputs
        .into_iter()
        .map(convert_transaction_input)
        .collect::<Result<_, _>>()
        .map_err(E::Inputs)?;
    let output = json
        .outputs
        .into_iter()
        .map(convert_transaction_output)
        .collect::<Result<_, _>>()
        .map_err(E::Outputs)?;

    Ok(bitcoin::Transaction { version, lock_time, input, output })
}

fn convert_transaction_input(input: RawTransactionInput) -> Result<TxIn, RawTransactionInputError> {
    use hex::FromHex as _;
    use RawTransactionInputError as E;

    let txid = input.txid.parse::<Txid>().map_err(E::Txid)?;
    let script_sig = input.script_sig.script_buf().map_err(E::ScriptSig)?;

    let witness = match input.txin_witness {
        None => Witness::new(),
        Some(v) => {
            // TODO: Add a constructor method on the `Witness` type that takes a list of hex strings.
            let bytes: Vec<Vec<u8>> = v
                .into_iter()
                .map(|hex| Vec::from_hex(&hex))
                .collect::<Result<_, _>>()
                .map_err(E::Witness)?;
            Witness::from_slice(&bytes)
        }
    };

    Ok(TxIn {
        previous_output: OutPoint { txid, vout: input.vout },
        script_sig,
        sequence: Sequence::from_consensus(input.sequence),
        witness,
    })
}

fn convert_transaction_output(
    output: RawTransactionOutput,
) -> Result<TxOut, RawTransactionOutputError> {
    use RawTransactionOutputError as E;

    let value = Amount::from_btc(output.value).map_err(E::Value)?;
    let script_pubkey = output.script_pubkey.script_buf().map_err(E::ScriptPubkey)?;

    Ok(TxOut { value, script_pubkey })
}

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
    pub fn into_model(self) -> Result<model::DecodePsbt, PsbtParseError> {
        todo!("Implement `into_model` for `DecodePsbt`.")
    }
}

impl DecodeRawTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::DecodeRawTransaction, RawTransactionError> {
        let raw_tx = convert_transaction(self.0)?;
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
            None => None,
            Some(hex) => Some(ScriptBuf::from_hex(&hex).map_err(E::Hex)?),
        };
        let addresses = match self.addresses {
            None => None,
            Some(addresses) => {
                let addresses = addresses
                    .iter()
                    .map(|s| s.parse::<Address<_>>())
                    .collect::<Result<_, _>>()
                    .map_err(E::Addresses)?;
                Some(addresses)
            }
        };
        let p2sh = self.p2sh.map(|s| s.parse::<Address<_>>()).transpose().map_err(E::P2sh)?;

        Ok(model::DecodeScript {
            script_pubkey,
            type_: self.type_,
            required_signatures: self.required_signatures,
            addresses,
            p2sh,
        })
    }
}

impl FinalizePsbt {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::FinalizePsbt, ()> { todo!() }
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
            .map(convert_transaction_input)
            .collect::<Result<_, _>>()
            .map_err(E::Inputs)?;
        let output = self
            .outputs
            .into_iter()
            .map(convert_transaction_output)
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
        let results = self.results.into_iter().map(|r| r.into_model()).collect::<Result<_, _>>()?;

        Ok(model::TestMempoolAccept { results })
    }
}

impl MempoolAcceptance {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::MempoolAcceptance, hex::HexToArrayError> {
        let txid = self.txid.parse::<Txid>()?;

        Ok(model::MempoolAcceptance {
            txid,
            allowed: self.allowed,
            reject_reason: self.reject_reason,
        })
    }
}
