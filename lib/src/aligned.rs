use std::str::FromStr;
use aligned_sdk::common::types::{AlignedVerificationData, FeeEstimationType, Network, ProvingSystemId, VerificationData};
use aligned_sdk::verification_layer::{estimate_fee, get_chain_id, get_nonce_from_batcher, submit_and_wait_verification};
use ethers::signers::{LocalWallet, Signer};
use tracing::{debug};

use crate::sp1::{Proof, SUDOKU_ELF};

pub struct AlignedClient {
    rpc_url: String,
    network: Network,
    wallet: LocalWallet,
}

impl AlignedClient {
    pub async fn new(rpc_url: String, network: Network, private_key: &str) -> Result<Self, String> {
        let mut wallet = LocalWallet::from_str(private_key)
            .map_err(|e| format!("Failed to create wallet: {}", e))?;
        let chain_id = get_chain_id(&rpc_url)
            .await
            .map_err(|e| format!("Failed to get chain ID: {}", e))?;
        wallet = wallet.with_chain_id(chain_id);
        Ok(Self {
            rpc_url,
            network,
            wallet,
        })
    }

    pub async fn send_proof(
        &self,
        proof: Proof
    ) -> Result<AlignedVerificationData, String> {
        let verification_data = VerificationData {
            proving_system: ProvingSystemId::SP1,
            proof: proof.proof,
            pub_input: Some(proof.public_values),
            verification_key: Some(proof.vk.to_vec()),
            vm_program_code: Some(SUDOKU_ELF.to_vec()),
            proof_generator_addr: self.wallet.address(),
        };

        let max_fee = estimate_fee(&self.rpc_url, FeeEstimationType::Instant)
            .await
            .map_err(|e| {
                debug!("Failed to estimate fee: {:?}", e);
                format!("Failed to estimate fee: {}", e)
            })?;

        let nonce = get_nonce_from_batcher(self.network.clone(), self.wallet.address())
            .await
            .map_err(|e| {
                debug!("Failed to get nonce from batcher: {:?}", e);
                format!("Failed to get nonce: {:?}", e)
            })?;

        let aligned_verification_data = submit_and_wait_verification(
            &self.rpc_url,
            self.network.clone(),
            &verification_data,
            max_fee,
            self.wallet.clone(), 
            nonce
        )
            .await
            .map_err(|e| {
                debug!("Failed to submit and wait for verification: {:?}", e);
                format!("Failed to submit and wait for verification: {}", e)
            })?;
        
        Ok(aligned_verification_data)
    }
}
