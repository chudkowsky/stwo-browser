use axum::{extract::State, routing::{get, post}, serve, Json, Router};
use serde::{Deserialize, Serialize};
use server::verify_state_machine;
use tokio::{net::TcpListener, sync::Mutex};
use core::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
 
#[derive(Deserialize)]
struct ProofRequest {
    username: String,
    proof: String, // Serialized proof data
    channel: String, // Serialized channel data
}
struct User{
    username: String,
    password: u32,
}
#[derive(Clone)]
struct AppState{
    users: Arc<Mutex<Vec<User>>>,
}

#[derive(Serialize)]
struct ProofResponse {
    valid: bool,
}

async fn verify_proof(State(app_state):State<AppState>,Json(payload): Json<ProofRequest>) -> Json<ProofResponse> {
    let extracted_password = verify_state_machine(payload.channel,payload.proof).unwrap();
    let username = payload.username;
    let mut users = app_state.users.lock().await;
    let user_exist = users.iter().any(|user| user.username == username);
    let is_valid = if user_exist{
        users.iter().any(|user| user.password == extracted_password && user.username == username)
    }else{
        users.push(User{username:username.clone(),password:extracted_password});
        true
    };   
    Json(ProofResponse { valid: is_valid })
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();
    let app_state = AppState{
        users: Arc::new(Mutex::new(Vec::new())),
    };
    let app = Router::new()
        .route("/verify", post(verify_proof))
        .route("/ok", get(|| async { "ok" }))
        .with_state(app_state)
        .layer(cors);
    let address: SocketAddr = format!("127.0.0.1:3000").parse().unwrap();
    let listner = TcpListener::bind(address).await.unwrap();
    println!("Listening on http://{}", address);
    serve(listner, app.into_make_service()).await.unwrap();
}
