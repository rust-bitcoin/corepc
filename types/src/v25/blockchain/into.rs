// SPDX-License-Identifier: CC0-1.0

use bitcoin::{Amount, BlockHash, FeeRate, ScriptBuf, Txid, Weight};

use super::error::ScanBlocksStartError;
use super::{GetBlockStats, GetBlockStatsError, ScanBlocksStart, ScanTxOutSetError, ScanTxOutSetStart,
    ScanTxOutSetUnspent};
use crate::model;

impl GetBlockStats {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockStats, GetBlockStatsError> {
        use GetBlockStatsError as E;

        // `FeeRate::sat_per_vb` returns an option if value overflows.
        let average_fee_rate = FeeRate::from_sat_per_vb(self.average_fee_rate);
        let block_hash = self.block_hash.parse::<BlockHash>().map_err(E::BlockHash)?;
        let fee_rate_percentiles = self
            .fee_rate_percentiles
            .iter()
            .map(|vb| FeeRate::from_sat_per_vb(*vb))
            .collect::<Vec<Option<FeeRate>>>();
        let max_fee_rate = FeeRate::from_sat_per_vb(self.max_fee_rate);
        let minimum_fee_rate = FeeRate::from_sat_per_vb(self.minimum_fee_rate);

        // FIXME: Double check that these values are virtual bytes and not weight units.
        let segwit_total_weight = Weight::from_vb(self.segwit_total_weight);
        let total_weight = Weight::from_vb(self.total_weight);

        Ok(model::GetBlockStats {
            average_fee: Amount::from_sat(self.average_fee),
            average_fee_rate,
            average_tx_size: crate::to_u32(self.average_tx_size, "average_tx_size")?,
            block_hash,
            fee_rate_percentiles,
            height: crate::to_u32(self.height, "height")?,
            inputs: crate::to_u32(self.inputs, "inputs")?,
            max_fee: Amount::from_sat(self.max_fee),
            max_fee_rate,
            max_tx_size: crate::to_u32(self.max_tx_size, "max_tx_size")?,
            median_fee: Amount::from_sat(self.median_fee),
            median_time: crate::to_u32(self.median_time, "median_time")?,
            median_tx_size: crate::to_u32(self.median_tx_size, "median_tx_size")?,
            minimum_fee: Amount::from_sat(self.minimum_fee),
            minimum_fee_rate,
            minimum_tx_size: crate::to_u32(self.minimum_tx_size, "minimum_tx_size")?,
            outputs: crate::to_u32(self.outputs, "outputs")?,
            subsidy: Amount::from_sat(self.subsidy),
            segwit_total_size: crate::to_u32(self.segwit_total_size, "segwit_total_size")?,
            segwit_total_weight,
            segwit_txs: crate::to_u32(self.segwit_txs, "segwit_txs")?,
            time: crate::to_u32(self.time, "time")?,
            total_out: Amount::from_sat(self.total_out),
            total_size: crate::to_u32(self.total_size, "total_size")?,
            total_weight,
            total_fee: Amount::from_sat(self.total_fee),
            txs: crate::to_u32(self.txs, "txs")?,
            utxo_increase: self.utxo_increase,
            utxo_size_increase: self.utxo_size_increase,
            utxo_increase_actual: self.utxo_increase_actual,
            utxo_size_increase_actual: self.utxo_size_increase_actual,
        })
    }
}

impl ScanBlocksStart {
    pub fn into_model(self) -> Result<model::ScanBlocksStart, ScanBlocksStartError> {
        let relevant_blocks = self
            .relevant_blocks
            .iter()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()
            .map_err(ScanBlocksStartError::RelevantBlocks)?;

        Ok(model::ScanBlocksStart {
            from_height: crate::to_u32(self.from_height, "from_height")?,
            to_height: crate::to_u32(self.to_height, "to_height")?,
            relevant_blocks,
        })
    }
}

impl ScanTxOutSetStart {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ScanTxOutSetStart, ScanTxOutSetError> {
        use ScanTxOutSetError as E;

        let bestblock = self.best_block.parse::<BlockHash>().map_err(E::BestBlockHash)?;

        let unspents = self
            .unspents
            .into_iter()
            .map(|u| u.into_model())
            .collect::<Result<Vec<_>, _>>()?;

        let total_amount = Amount::from_btc(self.total_amount).map_err(E::TotalAmount)?;

        Ok(model::ScanTxOutSetStart {
            success: Some(self.success),
            txouts: Some(self.txouts),
            height: Some(self.height),
            bestblock: Some(bestblock),
            unspents,
            total_amount,
        })
    }
}

impl ScanTxOutSetUnspent {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ScanTxOutSetUnspent, ScanTxOutSetError> {
        use ScanTxOutSetError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let amount = Amount::from_btc(self.amount).map_err(E::Amount)?;
        let script_pubkey = ScriptBuf::from_hex(&self.script_pubkey).map_err(E::ScriptPubKey)?;

        Ok(model::ScanTxOutSetUnspent {
            txid,
            vout: self.vout,
            script_pubkey,
            desc: Some(self.descriptor),
            amount,
            coinbase: Some(self.coinbase),
            height: self.height,
            blockhash: None,
            confirmations: None,
        })
    }
}
