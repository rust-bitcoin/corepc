// SPDX-License-Identifier: CC0-1.0

use bitcoin::{Amount, BlockHash, FeeRate, ScriptBuf, Txid, Weight};

use super::error::ScanBlocksStartError;
use super::{
    GetBlockStats, GetBlockStatsError, ScanBlocksStart, ScanTxOutSetError, ScanTxOutSetStart,
    ScanTxOutSetUnspent,
};
use crate::model;

impl GetBlockStats {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockStats, GetBlockStatsError> {
        use GetBlockStatsError as E;

        // `FeeRate::sat_per_vb` returns an option if value overflows.
        let average_fee_rate = self.average_fee_rate.and_then(FeeRate::from_sat_per_vb);
        let block_hash =
            self.block_hash.map(|h| h.parse::<BlockHash>()).transpose().map_err(E::BlockHash)?;
        let fee_rate_percentiles = self
            .fee_rate_percentiles
            .map(|arr| arr.iter().map(|vb| FeeRate::from_sat_per_vb(*vb)).collect());
        let max_fee_rate = self.max_fee_rate.and_then(FeeRate::from_sat_per_vb);
        let minimum_fee_rate = self.minimum_fee_rate.and_then(FeeRate::from_sat_per_vb);

        // FIXME: Double check that these values are virtual bytes and not weight units.
        let segwit_total_weight = self.segwit_total_weight.and_then(Weight::from_vb);
        let total_weight = self.total_weight.and_then(Weight::from_vb);

        Ok(model::GetBlockStats {
            average_fee: self.average_fee.map(Amount::from_sat),
            average_fee_rate,
            average_tx_size: self
                .average_tx_size
                .map(|v| crate::to_u32(v, "average_tx_size"))
                .transpose()?,
            block_hash,
            fee_rate_percentiles,
            height: self.height.map(|v| crate::to_u32(v, "height")).transpose()?,
            inputs: self.inputs.map(|v| crate::to_u32(v, "inputs")).transpose()?,
            max_fee: self.max_fee.map(Amount::from_sat),
            max_fee_rate,
            max_tx_size: self.max_tx_size.map(|v| crate::to_u32(v, "max_tx_size")).transpose()?,
            median_fee: self.median_fee.map(Amount::from_sat),
            median_time: self.median_time.map(|v| crate::to_u32(v, "median_time")).transpose()?,
            median_tx_size: self
                .median_tx_size
                .map(|v| crate::to_u32(v, "median_tx_size"))
                .transpose()?,
            minimum_fee: self.minimum_fee.map(Amount::from_sat),
            minimum_fee_rate,
            minimum_tx_size: self
                .minimum_tx_size
                .map(|v| crate::to_u32(v, "minimum_tx_size"))
                .transpose()?,
            outputs: self.outputs.map(|v| crate::to_u32(v, "outputs")).transpose()?,
            subsidy: self.subsidy.map(Amount::from_sat),
            segwit_total_size: self
                .segwit_total_size
                .map(|v| crate::to_u32(v, "segwit_total_size"))
                .transpose()?,
            segwit_total_weight,
            segwit_txs: self.segwit_txs.map(|v| crate::to_u32(v, "segwit_txs")).transpose()?,
            time: self.time.map(|v| crate::to_u32(v, "time")).transpose()?,
            total_out: self.total_out.map(Amount::from_sat),
            total_size: self.total_size.map(|v| crate::to_u32(v, "total_size")).transpose()?,
            total_weight,
            total_fee: self.total_fee.map(Amount::from_sat),
            txs: self.txs.map(|v| crate::to_u32(v, "txs")).transpose()?,
            utxo_increase: self.utxo_increase,
            utxo_size_increase: self.utxo_size_increase,
            utxo_increase_actual: self.utxo_increase_actual,
            utxo_size_increase_actual: self.utxo_size_increase_actual,
        })
    }
}

impl ScanBlocksStart {
    pub fn into_model(self) -> Result<model::ScanBlocksStart, ScanBlocksStartError> {
        use ScanBlocksStartError as E;

        let relevant_blocks = self
            .relevant_blocks
            .iter()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::RelevantBlocks)?;

        Ok(model::ScanBlocksStart {
            from_height: crate::to_u32(self.from_height, "from_height")?,
            to_height: crate::to_u32(self.to_height, "to_height")?,
            relevant_blocks,
            completed: None,
        })
    }
}

impl ScanTxOutSetStart {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::ScanTxOutSetStart, ScanTxOutSetError> {
        use ScanTxOutSetError as E;

        let best_block = self.best_block.parse::<BlockHash>().map_err(E::BestBlockHash)?;

        let unspents =
            self.unspents.into_iter().map(|u| u.into_model()).collect::<Result<Vec<_>, _>>()?;

        let total_amount = Amount::from_btc(self.total_amount).map_err(E::TotalAmount)?;

        Ok(model::ScanTxOutSetStart {
            success: self.success,
            tx_outs: Some(self.tx_outs),
            height: Some(self.height),
            best_block: Some(best_block),
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
            descriptor: Some(self.descriptor),
            amount,
            coinbase: Some(self.coinbase),
            height: self.height,
            block_hash: None,
            confirmations: None,
        })
    }
}
