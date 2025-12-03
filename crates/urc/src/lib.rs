mod bindings;

use alloy::primitives::{Address, B256, Bytes};
use alloy::sol_types::SolCall;
use blst::{
    BLST_ERROR, blst_bendian_from_fp, blst_fp, blst_p1_affine, blst_p1_uncompress, blst_p2_affine,
    blst_p2_uncompress,
};
use commit_boost::prelude::{BlsPublicKey, BlsSignature};
use eyre::Result;

use bindings::registry::BLS::{G1Point, G2Point};
use bindings::registry::IRegistry::SignedRegistration as SolSignedRegistration;
use bindings::registry::Registry::registerCall as SolRegisterCall;

/// URC registration message
pub struct Registration {
    pub owner: Address,
}

/// Signed registration used for URC.register
pub struct SignedRegistration {
    pub pubkey: BlsPublicKey,
    pub signature: BlsSignature,
    pub nonce: u64,
}

impl SignedRegistration {
    pub fn as_sol_type(&self) -> Result<SolSignedRegistration> {
        let pubkey = convert_pubkey_to_g1_point(&self.pubkey)?;
        let signature = convert_signature_to_g2_point(&self.signature)?;
        let registration = SolSignedRegistration {
            pubkey,
            signature,
            nonce: self.nonce,
        };
        Ok(registration)
    }
}

/// Container for URC register() call parameters
pub struct URCRegisterInputs {
    pub registrations: Vec<SignedRegistration>,
    pub owner: Address,
    pub signing_id: B256,
}

impl URCRegisterInputs {
    /// ABI encode for URC register() call
    /// Signature: register(SignedRegistration[] calldata registrations, address owner, bytes32 signingId)
    pub fn abi_encode_with_selector(&self) -> Result<Bytes> {
        let sol_registrations = self
            .registrations
            .iter()
            .map(|r| r.as_sol_type())
            .collect::<Result<Vec<_>, _>>()?;

        let register_call = SolRegisterCall {
            registrations: sol_registrations,
            owner: self.owner,
            signingId: self.signing_id,
        };

        let encoded = register_call.abi_encode();
        Ok(Bytes::from(encoded))
    }
}

/// Converts a pubkey to its corresponding affine G1 point form for EVM precompile usage
fn convert_pubkey_to_g1_point(pubkey: &BlsPublicKey) -> Result<G1Point> {
    let pubkey_byes = pubkey.serialize();
    let mut pubkey_affine = blst_p1_affine::default();
    let uncompress_result = unsafe { blst_p1_uncompress(&mut pubkey_affine, pubkey_byes.as_ptr()) };
    match uncompress_result {
        BLST_ERROR::BLST_SUCCESS => Ok(()),
        _ => Err(eyre::eyre!(
            "Error converting pubkey to affine point: {uncompress_result:?}"
        )),
    }?;
    let (x_a, x_b) = convert_fp_to_uint256_pair(&pubkey_affine.x);
    let (y_a, y_b) = convert_fp_to_uint256_pair(&pubkey_affine.y);
    Ok(G1Point { x_a, x_b, y_a, y_b })
}

/// Converts a signature to its corresponding affine G2 point form for EVM precompile usage
fn convert_signature_to_g2_point(signature: &BlsSignature) -> Result<G2Point> {
    let signature_bytes = signature.serialize();
    let mut signature_affine = blst_p2_affine::default();
    let uncompress_result =
        unsafe { blst_p2_uncompress(&mut signature_affine, signature_bytes.as_ptr()) };
    match uncompress_result {
        BLST_ERROR::BLST_SUCCESS => Ok(()),
        _ => Err(eyre::eyre!(
            "Error converting signature to affine point: {uncompress_result:?}"
        )),
    }?;
    let (x_c0_a, x_c0_b) = convert_fp_to_uint256_pair(&signature_affine.x.fp[0]);
    let (x_c1_a, x_c1_b) = convert_fp_to_uint256_pair(&signature_affine.x.fp[1]);
    let (y_c0_a, y_c0_b) = convert_fp_to_uint256_pair(&signature_affine.y.fp[0]);
    let (y_c1_a, y_c1_b) = convert_fp_to_uint256_pair(&signature_affine.y.fp[1]);
    Ok(G2Point {
        x_c0_a,
        x_c0_b,
        x_c1_a,
        x_c1_b,
        y_c0_a,
        y_c0_b,
        y_c1_a,
        y_c1_b,
    })
}

/// Converts a blst_fp to a pair of B256, as used in G1Point
fn convert_fp_to_uint256_pair(fp: &blst_fp) -> (B256, B256) {
    let mut fp_bytes = [0u8; 48];
    unsafe {
        blst_bendian_from_fp(fp_bytes.as_mut_ptr(), fp);
    }
    let mut high_bytes = [0u8; 32];
    high_bytes[16..].copy_from_slice(&fp_bytes[0..16]);
    let high = B256::from(high_bytes);
    let mut low_bytes = [0u8; 32];
    low_bytes.copy_from_slice(&fp_bytes[16..48]);
    let low = B256::from(low_bytes);
    (high, low)
}
