use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use lib::sp1::generate_proof;
use sudoku::is_valid_sudoku_solution;

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
        return HttpResponse::BadRequest().body("Bad Request: Invalid input");
    }

    // Generate the proof.
    let proof = match generate_proof(initial_state, solution) {
        Ok(proof) => proof,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", e)),
    };

    // Send proof to aligned client.

    HttpResponse::Ok().json(proof)
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