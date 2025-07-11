// SPDX-License-Identifier: CC0-1.0

use bitcoin::{Address, Amount, Txid, Wtxid};

use super::{
    DecodeScript, DecodeScriptError, DecodeScriptSegwit, DecodeScriptSegwitError,
    MempoolAcceptance, MempoolAcceptanceError, TestMempoolAccept, TestMempoolAcceptError,
};
use crate::model;

impl DecodeScript {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::DecodeScript, DecodeScriptError> {
        use DecodeScriptError as E;

        let address = match self.address {
            Some(addr) => Some(addr.parse::<Address<_>>().map_err(E::Address)?),
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
            script_pubkey: None,
            type_: self.type_,
            descriptor: None,
            address,
            required_signatures: self.required_signatures,
            addresses,
            p2sh,
            segwit: self.segwit.map(|s| s.into_model()).transpose().map_err(E::Segwit)?,
        })
    }
}

impl DecodeScriptSegwit {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::DecodeScriptSegwit, DecodeScriptSegwitError> {
        use DecodeScriptSegwitError as E;

        let address = match self.address {
            Some(addr) => Some(addr.parse::<Address<_>>().map_err(E::Address)?),
            None => None,
        };
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
            type_: self.type_,
            address,
            required_signatures,
            addresses,
            p2sh_segwit,
        })
    }
}

impl TestMempoolAccept {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::TestMempoolAccept, TestMempoolAcceptError> {
        let results = self.0.into_iter().map(|r| r.into_model()).collect::<Result<_, _>>()?;

        Ok(model::TestMempoolAccept { results })
    }
}

impl MempoolAcceptance {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::MempoolAcceptance, MempoolAcceptanceError> {
        use MempoolAcceptanceError as E;

        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;
        let wtxid = self.wtxid.parse::<Wtxid>().map_err(E::Wtxid)?;
        let vsize = self.vsize.map(|s| crate::to_u32(s, "vsize")).transpose()?;
        let fees = match self.fees {
            Some(s) => {
                let base = Amount::from_btc(s.base).map_err(E::Base)?;
                Some(model::MempoolAcceptanceFees {
                    base,
                    effective_feerate: None,  // v25 and later only.
                    effective_includes: None, // v25 and later only.
                })
            }
            None => None,
        };

        Ok(model::MempoolAcceptance {
            txid,
            wtxid: Some(wtxid),
            allowed: self.allowed,
            vsize,
            fees,
            reject_reason: self.reject_reason,
            reject_details: None, // v29 and later only.
        })
    }
}
