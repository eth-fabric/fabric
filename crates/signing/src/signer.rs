// use alloy::primitives::{Address, B256};
// use commit_boost::prelude::{
// 	BlsPublicKey, StartCommitModuleConfig,
// 	commit::{
// 		request::{SignConsensusRequest, SignProxyRequest},
// 		response::{BlsSignResponse, EcdsaSignResponse},
// 	},
// 	verify_proposer_commitment_signature_bls_for_message, verify_proposer_commitment_signature_ecdsa_for_message,
// };
// use eyre::{Result, WrapErr};
// use tracing::{debug, error, info};

// /// Calls the proxy_ecdsa signer to sign a hash
// pub async fn call_proxy_ecdsa_signer<T>(
// 	commit_config: &mut StartCommitModuleConfig<T>,
// 	message_hash: B256,
// 	committer: Address,
// 	module_signing_id: &B256,
// ) -> Result<EcdsaSignResponse> {
// 	debug!("Calling proxy_ecdsa signer for message hash: {:?}", message_hash);

// 	let proxy_request_ecdsa = SignProxyRequest::builder(committer).with_root(message_hash);

// 	// Make the actual API call to the signer service
// 	let proxy_response_ecdsa = commit_config
// 		.signer_client
// 		.request_proxy_signature_ecdsa(proxy_request_ecdsa)
// 		.await
// 		.wrap_err("Failed to request proxy signature from signer service")?;

// 	match verify_proposer_commitment_signature_ecdsa_for_message(
// 		commit_config.chain,
// 		&committer,
// 		&message_hash,
// 		&proxy_response_ecdsa.signature,
// 		module_signing_id,
// 		proxy_response_ecdsa.nonce,
// 	) {
// 		Ok(_) => info!("Signature verified successfully"),
// 		Err(err) => error!(%err, "Signature verification failed"),
// 	};

// 	Ok(proxy_response_ecdsa)
// }

// /// Calls the proxy_bls signer to sign a hash
// pub async fn call_proxy_bls_signer<T>(
// 	commit_config: &mut StartCommitModuleConfig<T>,
// 	message_hash: B256,
// 	bls_public_key: BlsPublicKey,
// 	module_signing_id: &B256,
// ) -> Result<BlsSignResponse> {
// 	debug!("Calling proxy_bls signer for message hash: {:?}", message_hash);

// 	let proxy_request_bls = SignProxyRequest::builder(bls_public_key.clone()).with_root(message_hash);

// 	// Make the actual API call to the signer service
// 	let proxy_response_bls = commit_config
// 		.signer_client
// 		.request_proxy_signature_bls(proxy_request_bls)
// 		.await
// 		.wrap_err("Failed to request proxy BLS signature from signer service")?;

// 	match verify_proposer_commitment_signature_bls_for_message(
// 		commit_config.chain,
// 		&bls_public_key,
// 		&message_hash,
// 		&proxy_response_bls.signature,
// 		module_signing_id,
// 		proxy_response_bls.nonce,
// 	) {
// 		true => info!("Signature verified successfully"),
// 		false => error!("Signature verification failed"),
// 	};

// 	Ok(proxy_response_bls)
// }

// /// Calls the BLS signer to sign a hash using the consensus key (not proxy)
// pub async fn call_bls_signer<T>(
// 	commit_config: &mut StartCommitModuleConfig<T>,
// 	message_hash: B256,
// 	bls_public_key: BlsPublicKey,
// 	module_signing_id: &B256,
// ) -> Result<BlsSignResponse> {
// 	debug!("Calling BLS signer for message hash: {:?} with consensus key", message_hash);

// 	// Build the consensus signature request
// 	let consensus_request = SignConsensusRequest::builder(bls_public_key.clone()).with_root(message_hash);

// 	// Make the actual API call to the signer service using consensus signature
// 	let bls_response = commit_config
// 		.signer_client
// 		.request_consensus_signature(consensus_request)
// 		.await
// 		.wrap_err("Failed to request consensus BLS signature from signer service")?;

// 	match verify_proposer_commitment_signature_bls_for_message(
// 		commit_config.chain,
// 		&bls_public_key,
// 		&message_hash,
// 		&bls_response.signature,
// 		module_signing_id,
// 		bls_response.nonce,
// 	) {
// 		true => info!("Consensus signature verified successfully"),
// 		false => error!("Consensus signature verification failed"),
// 	};

// 	Ok(bls_response)
// }

// /// Generates a proxy key using the signer client
// pub async fn generate_proxy_key_ecdsa<T>(
// 	commit_config: &mut StartCommitModuleConfig<T>,
// 	bls_public_key: BlsPublicKey,
// ) -> Result<Address> {
// 	debug!("Generating ECDSA proxy key for BLS public key: {:?}", bls_public_key);

// 	let signed_delegation = commit_config
// 		.signer_client
// 		.generate_proxy_key_ecdsa(bls_public_key)
// 		.await
// 		.wrap_err("Failed to generate ECDSA proxy key")?;

// 	let proxy_address = signed_delegation.message.proxy;
// 	info!("Generated ECDSA proxy key: {:?}", proxy_address);
// 	Ok(proxy_address)
// }

// /// Generates a BLS proxy key using the signer client
// pub async fn generate_proxy_key_bls<T>(
// 	commit_config: &mut StartCommitModuleConfig<T>,
// 	bls_public_key: BlsPublicKey,
// ) -> Result<BlsPublicKey> {
// 	debug!("Generating BLS proxy key for BLS public key: {:?}", bls_public_key);

// 	let signed_delegation = commit_config
// 		.signer_client
// 		.generate_proxy_key_bls(bls_public_key)
// 		.await
// 		.wrap_err("Failed to generate BLS proxy key")?;

// 	let proxy_key = signed_delegation.message.proxy;
// 	info!("Generated BLS proxy key: {:?}", proxy_key);
// 	Ok(proxy_key)
// }
