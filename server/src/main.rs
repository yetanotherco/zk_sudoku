use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::middleware::Logger;
use serde::{Deserialize, Serialize};
use lib::aligned;
use lib::sp1::generate_proof;
use tracing::{error, info, warn};
use tracing::subscriber::set_global_default;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use sudoku::is_valid_sudoku_solution;
use actix_cors::Cors;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sudoku {
    pub initial_state: String,
    pub solution: String,
}

async fn check_solution(
    body: web::Json<Sudoku>
) -> impl Responder {
    let initial_state = body.initial_state.as_str();
    let solution = body.solution.as_str();

    // Validate the Sudoku solution.
    if !is_valid_sudoku_solution(initial_state, solution) {
        warn!("Invalid Sudoku solution: {} with initial state: {}", solution, initial_state);
        return HttpResponse::BadRequest().json("Bad Request: Invalid input");
    }
    info!("Valid Sudoku solution: {} with initial state: {}",
          solution, initial_state);

    info!("Generating proof...");
    // Generate the proof.
    let proof = match generate_proof(initial_state, solution) {
        Ok(proof) => proof,
        Err(e) => {
            error!("Failed to generate proof: {}", e);
            return HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", e))
        },
    };
    info!("Proof generated successfully");

    // Send proof to aligned client.
    info!("Sending proof to aligned client...");
    match aligned::send_proof(proof.clone()).await {
        Ok(aligned_verification_data) => {
            info!("Proof sent successfully, aligned verification data: {:?}", aligned_verification_data);
            HttpResponse::Ok().json(aligned_verification_data)
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing();

    HttpServer::new(|| {
        App::new()
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