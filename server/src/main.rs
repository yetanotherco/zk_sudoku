use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sudoku_lib::sudoku::is_valid_sudoku_solution;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_elf!("sudoku-program");

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sudoku {
    pub initial_state: String,
    pub solution: String,
}

async fn check_solution(
    body: web::Json<Sudoku>
) -> impl Responder {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&body.initial_state);
    stdin.write(&body.solution);

    // Setup the program for proving.
    let (pk, vk) = client.setup(FIBONACCI_ELF);

    // Generate the proof
    let proof = client
        .prove(&pk, &stdin)
        .run()
        .expect("failed to generate proof");

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");

    format!(
        "Is the solution valid? {}",
        is_valid_sudoku_solution(body.initial_state.clone(), body.solution.clone())
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/check_solution", web::post().to(check_solution))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}