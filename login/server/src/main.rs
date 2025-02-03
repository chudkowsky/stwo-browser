use axum::{routing::{get, post}, serve, Json, Router};
use serde::{Deserialize, Serialize};
use prover::verify_fibonacci;
use tokio::net::TcpListener;
use core::net::SocketAddr;
use tower_http::cors::CorsLayer;
 
#[derive(Deserialize)]
struct ProofRequest {
    proof: String, // Serialized proof data
}

#[derive(Serialize)]
struct ProofResponse {
    valid: bool,
}

async fn verify_proof(Json(payload): Json<ProofRequest>) -> Json<ProofResponse> {
    let is_valid = verify_fibonacci(payload.proof);
    Json(ProofResponse { valid: is_valid })
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();
    let app = Router::new()
        .route("/verify", post(verify_proof))
        .route("/ok", get(|| async { "ok" }))
        .layer(cors);
    let address: SocketAddr = format!("127.0.0.1:3000").parse().unwrap();
    let listner = TcpListener::bind(address).await.unwrap();
    println!("Listening on http://{}", address);
    serve(listner, app.into_make_service()).await.unwrap();
}
    