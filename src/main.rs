// src/main.rs
use axum_server::make_app;
use dotenvy::dotenv;
use std::net::SocketAddr; // crate name as in Cargo.toml

#[tokio::main]
async fn main() {
    // init env + logging
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let app = make_app();

    tracing::info!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
