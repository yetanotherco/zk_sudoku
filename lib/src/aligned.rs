use std::str::FromStr;
use aligned_sdk::common::types::{AlignedVerificationData, FeeEstimationType, Network, ProvingSystemId, VerificationData};
use aligned_sdk::verification_layer::{estimate_fee, get_chain_id, get_nonce_from_batcher, submit_and_wait_verification};
use ethers::signers::{LocalWallet, Signer};
use tracing::{debug};


use crate::sp1::{Proof, SUDOKU_ELF};

const ANVIL_PRIVATE_KEY: &str = "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6"; // Anvil address 9

pub async fn send_proof(
    proof: Proof
) -> Result<AlignedVerificationData, String> {
    let rpc_url = "http://localhost:8545";
    
    let mut wallet = LocalWallet::from_str(ANVIL_PRIVATE_KEY)
        .map_err(|e| format!("Failed to create wallet: {}", e))?;
    let chain_id = get_chain_id(rpc_url)
        .await
        .map_err(|e| format!("Failed to get chain ID: {}", e))?;
    wallet = wallet.with_chain_id(chain_id);

    let verification_data = VerificationData {
        proving_system: ProvingSystemId::SP1,
        proof: proof.proof,
        pub_input: Some(proof.public_values),
        verification_key: Some(proof.vk.to_vec()),
        vm_program_code: Some(SUDOKU_ELF.to_vec()),
        proof_generator_addr: wallet.address(),
    };

    let max_fee = estimate_fee(rpc_url, FeeEstimationType::Instant)
        .await
        .map_err(|e| {
            debug!("Failed to estimate fee: {:?}", e);
            format!("Failed to estimate fee: {}", e)
        })?;

    let nonce = get_nonce_from_batcher(Network::Devnet, wallet.address())
        .await
        .map_err(|e| {
            debug!("Failed to get nonce from batcher: {:?}", e);
            format!("Failed to get nonce: {:?}", e)
        })?;

    let aligned_verification_data = submit_and_wait_verification(
        rpc_url,
        Network::Devnet,
        &verification_data,
        max_fee,
        wallet,
        nonce
    )
        .await
        .map_err(|e| {
            debug!("Failed to submit and wait for verification: {:?}", e);
            format!("Failed to submit and wait for verification: {}", e)
        })?;
    
    Ok(aligned_verification_data)
}
