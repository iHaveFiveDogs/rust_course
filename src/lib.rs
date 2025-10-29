// in src/lib.rs (create this file) - small pure function
use axum::{
    Router,
    routing::{get, post},
};

mod handlers;
pub use handlers::*; // optional re-export for tests

pub fn make_app() -> Router {
    Router::new()
        .route("/", get(|| async { "Axum server is running!" }))
        .route("/ping", get(ping))
        .route("/echo", post(echo))
        .route("/greet/:name", get(greet))
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
