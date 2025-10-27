// src/handlers.rs
use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EchoRequest {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EchoResponse {
    pub message: String,
    pub len: usize,
}

pub async fn ping() -> &'static str {
    "pong"
}

pub async fn echo(Json(payload): Json<EchoRequest>) -> impl IntoResponse {
    let resp = EchoResponse {
        len: payload.message.len(),
        message: payload.message,
    };
    (StatusCode::OK, Json(resp))
}

pub async fn greet(Path(name): Path<String>) -> String {
    format!("hello, {}!", name)
}
