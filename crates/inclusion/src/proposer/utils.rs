use alloy::primitives::{Address, B256, Bytes};
use alloy::rpc::types::beacon::{BlsPublicKey, BlsSignature};
use eyre::Result;
use signing::signer;

use commit_boost::prelude::{Chain, commit::client::SignerClient};
use constraints::types::{Delegation, SignedDelegation};
use urc::utils::get_delegation_signing_root;

/// Sign a delegation message using the consensus BLS key
pub async fn create_signed_delegation(
    signer_client: &mut SignerClient,
    proposer_public_key: &BlsPublicKey,
    gateway_public_key: &BlsPublicKey,
    slot: u64,
    gateway_address: &Address,
    module_signing_id: &B256,
    chain: &Chain,
) -> Result<SignedDelegation> {
    let delegation = Delegation {
        proposer: proposer_public_key.clone(),
        delegate: gateway_public_key.clone(),
        committer: gateway_address.clone(),
        slot,
        metadata: Bytes::new(),
    };

    let signing_root = get_delegation_signing_root(&delegation)?;

    // Sign using the signer client
    let response = signer::call_bls_signer(
        signer_client,
        signing_root,
        proposer_public_key.clone(),
        module_signing_id,
        chain.clone(),
    )
    .await?;

    Ok(SignedDelegation {
        message: delegation.clone(),
        nonce: response.nonce,
        signing_id: response.module_signing_id,
        signature: BlsSignature::new(response.signature.serialize()),
    })
}
