// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::taproot::{IncompleteBuilderError, TaprootBuilderError, TaprootError};
use bitcoin::{bip32, hex, secp256k1, sighash};

use crate::error::write_err;
use crate::v17::{Bip32DerivError, PartialSignatureError, RawTransactionError, WitnessUtxoError};

/// Error when converting a `DecodePsbt` type into the model type.
#[derive(Debug)]
pub enum DecodePsbtError {
    /// Conversion of the `tx` field failed.
    Tx(RawTransactionError),
    /// Conversion of the `global_xpubs` field failed.
    GlobalXpubs(GlobalXpubError),
    /// Conversion of the `proprietary` field failed.
    Proprietary(hex::HexToBytesError),
    /// Conversion of one the map items in the `unknown` field failed.
    Unknown(hex::HexToBytesError),
    /// Conversion of one of the PSBT inputs failed.
    Inputs(PsbtInputError),
    /// Conversion of one of the PSBT outputs failed.
    Outputs(PsbtOutputError),
}

impl fmt::Display for DecodePsbtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DecodePsbtError as E;

        match *self {
            E::Tx(ref e) => write_err!(f, "conversion of raw transaction data field failed"; e),
            E::GlobalXpubs(ref e) => {
                write_err!(f, "conversion of one the map items in the `global_xbubs` field failed"; e)
            }
            E::Proprietary(ref e) => {
                write_err!(f, "conversion of one the map items in the `proprietray` field failed"; e)
            }
            E::Unknown(ref e) => {
                write_err!(f, "conversion of one the map items in the `unknown` field failed"; e)
            }
            E::Inputs(ref e) => write_err!(f, "conversion of one of the PSBT inputs failed"; e),
            E::Outputs(ref e) => write_err!(f, "conversion of one of the PSBT outputs failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodePsbtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use DecodePsbtError as E;

        match *self {
            E::Tx(ref e) => Some(e),
            E::GlobalXpubs(ref e) => Some(e),
            E::Proprietary(ref e) => Some(e),
            E::Unknown(ref e) => Some(e),
            E::Inputs(ref e) => Some(e),
            E::Outputs(ref e) => Some(e),
        }
    }
}

/// Error when converting one of the global xpubs failed.
#[derive(Debug)]
pub enum GlobalXpubError {
    /// Conversion of the `xpub` field failed.
    Xpub(bip32::Error),
    /// Conversion of the `master_fingerprint` field failed.
    MasterFingerprint(hex::HexToArrayError),
    /// Conversion of the `path` field failed.
    Path(bip32::Error),
}

impl fmt::Display for GlobalXpubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GlobalXpubError as E;

        match *self {
            E::Xpub(ref e) => write_err!(f, "conversion of the xpub failed"; e),
            E::MasterFingerprint(ref e) => {
                write_err!(f, "conversion of the `master_fingerprint` field failed"; e)
            }
            E::Path(ref e) => write_err!(f, "conversion of the `path` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GlobalXpubError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GlobalXpubError as E;

        match *self {
            E::Xpub(ref e) => Some(e),
            E::MasterFingerprint(ref e) => Some(e),
            E::Path(ref e) => Some(e),
        }
    }
}

/// Error when converting one of the `DecodePsbt` inputs failed.
#[derive(Debug)]
pub enum PsbtInputError {
    /// Conversion of the `non_witness_utxo` field failed.
    NonWitnessUtxo(RawTransactionError),
    /// Conversion of the `witness_utxo` field failed.
    WitnessUtxo(WitnessUtxoError),
    /// Conversion of the `partial_signatures` field failed.
    PartialSignatures(PartialSignatureError),
    /// Conversion of the `sighash` field failed.
    Sighash(sighash::SighashTypeParseError),
    /// Conversion of the `redeem_script` field failed.
    RedeemScript(hex::HexToBytesError),
    /// Conversion of the `witness_script` field failed.
    WitnessScript(hex::HexToBytesError),
    /// Conversion of the `bip32_derivs` field failed.
    Bip32Derivs(Bip32DerivError),
    /// Conversion of the `final_script_sig` field failed.
    FinalScriptSig(hex::HexToBytesError),
    /// Conversion of the `final_script_witness` field failed.
    FinalScriptWitness(hex::HexToBytesError),
    /// Conversion of the `ripemd160` hash failed.
    Ripemd160(hex::HexToArrayError),
    /// Conversion of the `ripemd160` preimage failed.
    Ripemd160Preimage(hex::HexToBytesError),
    /// Conversion of the `sha256` hash failed.
    Sha256(hex::HexToArrayError),
    /// Conversion of the `sha256` preimage failed.
    Sha256Preimage(hex::HexToBytesError),
    /// Conversion of the `hash160` hash failed.
    Hash160(hex::HexToArrayError),
    /// Conversion of the `hash160` preimage failed.
    Hash160Preimage(hex::HexToBytesError),
    /// Conversion of the `hash256` hash failed.
    Hash256(hex::HexToArrayError),
    /// Conversion of the `hash256` preimage failed.
    Hash256Preimage(hex::HexToBytesError),
    /// Conversion of the `taproot_key_path_sig` field failed.
    TaprootKeyPathSig(super::taproot::Error),
    /// Conversion of the `taproot_script_path_sigs` field failed.
    TaprootScriptPathSigs(TaprootScriptPathSigError),
    /// Conversion of the `taproot_scripts` field failed.
    TaprootScripts(TaprootScriptError),
    /// Conversion of the `taproot_bip32_derives` field failed.
    TaprootBip32Derivs(TaprootBip32DerivsError),
    /// Conversion of the `taproot_internal_key` field failed.
    TaprootInternalKey(secp256k1::Error),
    /// Conversion of the `taproot_merkle_root` field failed.
    TaprootMerkleRoot(hex::HexToArrayError),
    /// Conversion of the `proprietary` field failed.
    Proprietary(hex::HexToBytesError),
    /// Conversion of the `unknown` field failed.
    Unknown(hex::HexToBytesError),
}

impl fmt::Display for PsbtInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PsbtInputError as E;

        match *self {
            E::NonWitnessUtxo(ref e) => {
                write_err!(f, "conversion of the `non_witness_utxo` field failed"; e)
            }
            E::WitnessUtxo(ref e) => {
                write_err!(f, "conversion of the `witness_utxo` field failed"; e)
            }
            E::PartialSignatures(ref e) => {
                write_err!(f, "conversion of the `partial_signatures` field failed"; e)
            }
            E::Sighash(ref e) => write_err!(f, "conversion of the `sighash` field failed"; e),
            E::RedeemScript(ref e) => {
                write_err!(f, "conversion of the `redeem_script` field failed"; e)
            }
            E::WitnessScript(ref e) => {
                write_err!(f, "conversion of the `witness_script` field failed"; e)
            }
            E::Bip32Derivs(ref e) => {
                write_err!(f, "conversion of the `bip32_derivs` field failed"; e)
            }
            E::FinalScriptSig(ref e) => {
                write_err!(f, "conversion of the `final_script_sig` field failed"; e)
            }
            E::FinalScriptWitness(ref e) => {
                write_err!(f, "conversion of the `final_script_witness` field failed"; e)
            }
            E::Ripemd160(ref e) => write_err!(f, "conversion of the `ripemd160` hash failed"; e),
            E::Ripemd160Preimage(ref e) => {
                write_err!(f, "conversion of the `ripemd160` preimage failed"; e)
            }
            E::Sha256(ref e) => write_err!(f, "conversion of the `sha256` hash failed"; e),
            E::Sha256Preimage(ref e) => {
                write_err!(f, "conversion of the `sha256` preimage failed"; e)
            }
            E::Hash160(ref e) => write_err!(f, "conversion of the `hash160` hash failed"; e),
            E::Hash160Preimage(ref e) => {
                write_err!(f, "conversion of the `hash160` preimage failed"; e)
            }
            E::Hash256(ref e) => write_err!(f, "conversion of the `hash256` hash failed"; e),
            E::Hash256Preimage(ref e) => {
                write_err!(f, "conversion of the `hash256` preimage failed"; e)
            }
            E::TaprootKeyPathSig(ref e) => {
                write_err!(f, "conversion of the `taproot_key_path_sig` field failed"; e)
            }
            E::TaprootScriptPathSigs(ref e) => {
                write_err!(f, "conversion of the `taproot_script_path_sigs` field failed"; e)
            }
            E::TaprootScripts(ref e) => {
                write_err!(f, "conversion of the `taproot_scripts` field failed"; e)
            }
            E::TaprootBip32Derivs(ref e) => {
                write_err!(f, "conversion of the `taproot_bip32_derivs` field failed"; e)
            }
            E::TaprootInternalKey(ref e) => {
                write_err!(f, "conversion of the `taproot_internal_key` field failed"; e)
            }
            E::TaprootMerkleRoot(ref e) => {
                write_err!(f, "conversion of the `taproot_merkle_root` field failed"; e)
            }
            E::Proprietary(ref e) => {
                write_err!(f, "conversion of one the map items in the `proprietray` field failed"; e)
            }
            E::Unknown(ref e) => write_err!(f, "conversion of the `unknown` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PsbtInputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use PsbtInputError as E;

        match *self {
            E::NonWitnessUtxo(ref e) => Some(e),
            E::WitnessUtxo(ref e) => Some(e),
            E::PartialSignatures(ref e) => Some(e),
            E::Sighash(ref e) => Some(e),
            E::RedeemScript(ref e) => Some(e),
            E::WitnessScript(ref e) => Some(e),
            E::Bip32Derivs(ref e) => Some(e),
            E::FinalScriptSig(ref e) => Some(e),
            E::FinalScriptWitness(ref e) => Some(e),
            E::Ripemd160(ref e) => Some(e),
            E::Ripemd160Preimage(ref e) => Some(e),
            E::Sha256(ref e) => Some(e),
            E::Sha256Preimage(ref e) => Some(e),
            E::Hash160(ref e) => Some(e),
            E::Hash160Preimage(ref e) => Some(e),
            E::Hash256(ref e) => Some(e),
            E::Hash256Preimage(ref e) => Some(e),
            E::TaprootKeyPathSig(ref e) => Some(e),
            E::TaprootScriptPathSigs(ref e) => Some(e),
            E::TaprootScripts(ref e) => Some(e),
            E::TaprootBip32Derivs(ref e) => Some(e),
            E::TaprootInternalKey(ref e) => Some(e),
            E::TaprootMerkleRoot(ref e) => Some(e),
            E::Proprietary(ref e) => Some(e),
            E::Unknown(ref e) => Some(e),
        }
    }
}

/// Error when converting one of the `DecodePsbt` outputs failed.
#[derive(Debug)]
pub enum PsbtOutputError {
    /// Conversion of the `redeem_script` field failed.
    RedeemScript(hex::HexToBytesError),
    /// Conversion of the `witness_script` field failed.
    WitnessScript(hex::HexToBytesError),
    /// Conversion of the `bip32_derivs` field failed.
    Bip32Derivs(Bip32DerivError),
    /// Conversion of the `taproot_internal_key` field failed.
    TaprootInternalKey(secp256k1::Error),
    /// Conversion of the `taproot_tree` field failed.
    TaprootTree(TaprootLeafError),
    /// Conversion of the `taproot_bip32_derives` field failed.
    TaprootBip32Derivs(TaprootBip32DerivsError),
    /// Conversion of the `proprietary` field failed.
    Proprietary(hex::HexToBytesError),
    /// Conversion of the `unknown` field failed.
    Unknown(hex::HexToBytesError),
}

impl fmt::Display for PsbtOutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PsbtOutputError as E;

        match *self {
            E::RedeemScript(ref e) => {
                write_err!(f, "conversion of the `redeem_script` field failed"; e)
            }
            E::WitnessScript(ref e) => {
                write_err!(f, "conversion of the `witness_script` field failed"; e)
            }
            E::Bip32Derivs(ref e) => {
                write_err!(f, "conversion of the `bip32_derivs` field failed"; e)
            }
            E::TaprootInternalKey(ref e) => {
                write_err!(f, "conversion of the `taproot_internal_key` field failed"; e)
            }
            E::TaprootTree(ref e) => {
                write_err!(f, "conversion of the `taproot_tree` field failed"; e)
            }
            E::TaprootBip32Derivs(ref e) => {
                write_err!(f, "conversion of the `taproot_bip32_derivs` field failed"; e)
            }
            E::Proprietary(ref e) => {
                write_err!(f, "conversion of one the map items in the `proprietray` field failed"; e)
            }
            E::Unknown(ref e) => write_err!(f, "conversion of the `unknown` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PsbtOutputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use PsbtOutputError as E;

        match *self {
            E::RedeemScript(ref e) => Some(e),
            E::WitnessScript(ref e) => Some(e),
            E::Bip32Derivs(ref e) => Some(e),
            E::TaprootInternalKey(ref e) => Some(e),
            E::TaprootTree(ref e) => Some(e),
            E::TaprootBip32Derivs(ref e) => Some(e),
            E::Proprietary(ref e) => Some(e),
            E::Unknown(ref e) => Some(e),
        }
    }
}

/// Error when converting a taproot script path sig.
#[derive(Debug)]
pub enum TaprootScriptPathSigError {
    /// Conversion of the `pubkey` field failed.
    Pubkey(secp256k1::Error),
    /// Conversion of the `leaf_hash` field failed.
    LeafHash(hex::HexToArrayError),
    /// Conversion of the `sig` field failed.
    Sig(super::taproot::Error),
}

impl fmt::Display for TaprootScriptPathSigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TaprootScriptPathSigError as E;

        match *self {
            E::Pubkey(ref e) => write_err!(f, "conversion of the `pubkey` field failed"; e),
            E::LeafHash(ref e) => write_err!(f, "conversion of the `leaf_hash` field failed"; e),
            E::Sig(ref e) => write_err!(f, "conversion of the `sig` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TaprootScriptPathSigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use TaprootScriptPathSigError as E;

        match *self {
            E::Pubkey(ref e) => Some(e),
            E::LeafHash(ref e) => Some(e),
            E::Sig(ref e) => Some(e),
        }
    }
}

/// Error when converting a taproot script.
#[derive(Debug)]
pub enum TaprootScriptError {
    /// Conversion of the `script` field failed.
    Script(hex::HexToBytesError),
    /// Conversion of the `leaf_ver` field failed.
    LeafVer(TaprootError),
    /// Conversion of the `control_blocks` field failed.
    ControlBlocks(ControlBlocksError),
}

impl fmt::Display for TaprootScriptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TaprootScriptError as E;

        match *self {
            E::Script(ref e) => write_err!(f, "conversion of the `script` field failed"; e),
            E::LeafVer(ref e) => write_err!(f, "conversion of the `leaf_ver` field failed"; e),
            E::ControlBlocks(ref e) => {
                write_err!(f, "conversion of the `control_blocks` field failed"; e)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TaprootScriptError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use TaprootScriptError as E;

        match *self {
            E::Script(ref e) => Some(e),
            E::LeafVer(ref e) => Some(e),
            E::ControlBlocks(ref e) => Some(e),
        }
    }
}

/// Error when converting the control blocks vector.
#[derive(Debug)]
pub enum ControlBlocksError {
    /// No control block returned by Core for this script.
    Missing,
    /// Multiple control blocks returned by Core for this script.
    Multiple(usize),
    /// Failed to parse control block hex string.
    Parse(hex::HexToBytesError),
    /// Failed to decode parsed bytes.
    Decode(TaprootError),
}

impl fmt::Display for ControlBlocksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ControlBlocksError as E;

        match *self {
            E::Missing => write!(f, "no control block returned by Core for this script"),
            E::Multiple(n) => {
                write!(f, "multiple control blocks returned by Core for this script: {}", n)
            }
            E::Parse(ref e) => write_err!(f, "failed to parse control block hex"; e),
            E::Decode(ref e) => write_err!(f, "failed to decode control block from bytes"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ControlBlocksError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ControlBlocksError as E;

        match *self {
            E::Missing => None,
            E::Multiple(_) => None,
            E::Parse(ref e) => Some(e),
            E::Decode(ref e) => Some(e),
        }
    }
}

/// Error when converting a taproot BIP-32 derivation.
#[derive(Debug)]
pub enum TaprootBip32DerivsError {
    /// Conversion of the `pubkey` field failed.
    Pubkey(secp256k1::Error),
    /// Conversion of the `master_fingerprint` field failed.
    MasterFingerprint(hex::HexToArrayError),
    /// Conversion of the `path` field failed.
    Path(bip32::Error),
    /// Conversion of one of the leaf hashes failed.
    LeafHashes(hex::HexToArrayError),
}

impl fmt::Display for TaprootBip32DerivsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TaprootBip32DerivsError as E;

        match *self {
            E::Pubkey(ref e) => write_err!(f, "conversion of the `pubkey` field failed"; e),
            E::MasterFingerprint(ref e) => {
                write_err!(f, "conversion of the `master_fingerprint` field failed"; e)
            }
            E::Path(ref e) => write_err!(f, "conversion of the `path` field failed"; e),
            E::LeafHashes(ref e) => {
                write_err!(f, "conversion of the `leaf_hashes` field failed"; e)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TaprootBip32DerivsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use TaprootBip32DerivsError as E;

        match *self {
            E::Pubkey(ref e) => Some(e),
            E::MasterFingerprint(ref e) => Some(e),
            E::Path(ref e) => Some(e),
            E::LeafHashes(ref e) => Some(e),
        }
    }
}

/// Error when converting a taproot script.
#[derive(Debug)]
pub enum TaprootLeafError {
    /// Conversion of the `leaf_ver` field failed.
    LeafVer(TaprootError),
    /// Conversion of the `script` field failed.
    Script(hex::HexToBytesError),
    /// Failed to add leaf to builder.
    TaprootBuilder(TaprootBuilderError),
    /// Failed to convert builder into a tap tree.
    IncompleteBuilder(IncompleteBuilderError),
}

impl fmt::Display for TaprootLeafError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TaprootLeafError as E;

        match *self {
            E::LeafVer(ref e) => write_err!(f, "conversion of the `leaf_ver` field failed"; e),
            E::Script(ref e) => write_err!(f, "conversion of the `script` field failed"; e),
            E::TaprootBuilder(ref e) => write_err!(f, "failed to add leaf to builder"; e),
            E::IncompleteBuilder(ref e) => {
                write_err!(f, "failed to convert builder into a tap tree"; e)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TaprootLeafError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use TaprootLeafError as E;

        match *self {
            E::Script(ref e) => Some(e),
            E::LeafVer(ref e) => Some(e),
            E::TaprootBuilder(ref e) => Some(e),
            E::IncompleteBuilder(ref e) => Some(e),
        }
    }
}
