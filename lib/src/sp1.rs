use sp1_sdk::{HashableKey, include_elf, ProverClient, SP1Stdin};
use serde::{Deserialize, Serialize};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_elf!("sudoku-program");

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Proof {
    pub proof: Vec<u8>,
    pub public_values: Vec<u8>,
    pub vk: [u8; 32],
}

// TODO move to a different file (zk_helper.rs or similar)
pub fn generate_proof(initial_state: &str, solution: &str, ) -> Result<Proof, String> {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&initial_state.to_string());
    stdin.write(&solution.to_string());

    // Setup the program for proving.
    let (pk, vk) = client.setup(FIBONACCI_ELF);

    // Generate the proof
    let proof = client
        .prove(&pk, &stdin)
        .run()
        .map_err(|e| format!("failed to generate proof: {}", e))?;

    // Verify the proof.
    client.verify(&proof, &vk).map_err(|e| format!("failed to verify proof: {}", e))?;

    // Return Proof
    Ok(Proof {
        proof: bincode::serialize(&proof).expect("failed to serialize"),
        public_values: proof.public_values.to_vec(),
        vk: vk.hash_bytes(),
    })
}