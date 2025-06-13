use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::middleware::Logger;
use serde::{Deserialize, Serialize};
use lib::aligned::{AlignedClient}; // Import AlignedClient
use lib::sp1::generate_proof;
use tracing::{error, info, warn};
use tracing::subscriber::set_global_default;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use sudoku::is_valid_sudoku_solution;
use actix_cors::Cors;
use lib::{AlignedVerificationData, Network}; // Import Network from lib crate
use std::env;
use tokio::sync::{mpsc, oneshot};

// Default values for environment/configuration
const DEFAULT_RPC_URL: &str = "http://localhost:8545";
const DEFAULT_NETWORK: &str = "devnet";
const DEFAULT_ANVIL_PRIVATE_KEY: &str = "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sudoku {
    pub initial_state: String,
    pub solution: String,
}

// Add a struct for the response
#[derive(Debug, Serialize)]
pub struct ProofResponse {
    pub aligned_verification_data: AlignedVerificationData,
    pub link: String,
}

#[derive(Debug)]
pub struct ProofRequest {
    pub initial_state: String,
    pub solution: String,
    pub response: oneshot::Sender<Result<lib::sp1::Proof, String>>,
}

async fn request_proof(
    proof_tx: &mpsc::Sender<ProofRequest>,
    initial_state: &str,
    solution: &str,
) -> Result<lib::sp1::Proof, String> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let req = ProofRequest {
        initial_state: initial_state.to_string(),
        solution: solution.to_string(),
        response: resp_tx,
    };
    proof_tx.send(req).await.map_err(|_| "Proof generation queue unavailable".to_string())?;
    match resp_rx.await {
        Ok(Ok(proof)) => Ok(proof),
        Ok(Err(e)) => Err(e),
        Err(_) => Err("Proof generation task dropped".to_string()),
    }
}

async fn check_solution(
    body: web::Json<Sudoku>,
    aligned_client: web::Data<AlignedClient>,
    proof_tx: web::Data<mpsc::Sender<ProofRequest>>,
) -> impl Responder {
    let initial_state = body.initial_state.as_str();
    let solution = body.solution.as_str();

    // Validate the Sudoku solution.
    if !is_valid_sudoku_solution(initial_state, solution) {
        warn!("Invalid Sudoku solution: {} with initial state: {}", solution, initial_state);
        return HttpResponse::BadRequest().json("Invalid solution");
    }
    info!("Valid Sudoku solution: {} with initial state: {}",
          solution, initial_state);

    let proof = match request_proof(&proof_tx, initial_state, solution).await {
        Ok(proof) => proof,
        Err(e) => {
            error!("Failed to generate proof: {}", e);
            return HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", e));
        }
    };
    
    // Send proof to aligned client.
    info!("Sending proof to aligned client...");

    match aligned_client.send_proof(proof.clone()).await {
        Ok(aligned_verification_data) => {
            info!("Proof sent successfully, aligned verification data: {:?}", aligned_verification_data);
            let link = aligned_client.get_batch_url(aligned_verification_data.clone());
            HttpResponse::Ok().json(ProofResponse {
                aligned_verification_data,
                link
            })
        },
        Err(e) => {
            error!("Failed to send proof: {}", e);
            HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", e))
        },
    }
}

// Initialize tracing-subscriber
fn init_tracing() {
    let filter = EnvFilter::new("info,sp1_cuda=warn");
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
    set_global_default(subscriber).expect("setting default subscriber failed");
}

fn start_proof_generation_task() -> mpsc::Sender<ProofRequest> {
    let (proof_tx, mut proof_rx) = mpsc::channel::<ProofRequest>(8);
    tokio::spawn(async move {
        while let Some(req) = proof_rx.recv().await {
            info!("Generating proof for initial state: {}",
                  req.initial_state);
            let result = generate_proof(&req.initial_state, &req.solution);
            let _ = req.response.send(result);
            info!("Proof generation task completed for initial state: {}",
                  req.initial_state);
        }
    });
    proof_tx
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing();

    // Create the proof generation channel and background task
    let proof_tx = start_proof_generation_task();

    // Read RPC_URL and NETWORK from environment variables or command line args
    let args: Vec<String> = env::args().collect();
    let rpc_url = args.get(1)
        .map(|s| s.to_string())
        .or_else(|| env::var("RPC_URL").ok())
        .unwrap_or_else(|| DEFAULT_RPC_URL.to_string());
    let network_str = args.get(2)
        .map(|s| s.to_string())
        .or_else(|| env::var("NETWORK").ok())
        .unwrap_or_else(|| DEFAULT_NETWORK.to_string());
    let private_key = env::var("ANVIL_PRIVATE_KEY").unwrap_or_else(|_| DEFAULT_ANVIL_PRIVATE_KEY.to_string());
    let aligned_explorer_url = args.get(3)
        .map(|s| s.to_string())
        .or_else(|| env::var("ALIGNED_EXPLORER_URL").ok())
        .unwrap_or_else(|| "http://localhost:4000".to_string());

    let network = match network_str.to_lowercase().as_str() {
        "devnet" => Network::Devnet,
        "holesky" => Network::Holesky,
        "holesky-stage" => Network::HoleskyStage,
        "mainnet" => Network::Mainnet,
        _ => Network::Devnet,
    };

    // Create AlignedClient instance
    let aligned_client = match AlignedClient::new(rpc_url, aligned_explorer_url, network, &private_key).await {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to create AlignedClient: {}", e);
            // Exit if client creation fails, as it's essential for the server
            return Err(std::io::Error::other(format!("Failed to create AlignedClient: {}", e)));
        }
    };
    let aligned_client_data = web::Data::new(aligned_client);

    HttpServer::new(move || {
        App::new()
            .app_data(aligned_client_data.clone())
            .app_data(web::Data::new(proof_tx.clone()))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .route("/check_solution", web::post().to(check_solution))
    })
        .bind("127.0.0.1:9090")?
        .run()
        .await
}