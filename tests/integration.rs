// tests/integration.rs
use axum::body::Body;
use axum::http::Request;
use axum_server::make_app;
use tower::ServiceExt; // for `oneshot` // crate name matches Cargo.toml

#[tokio::test]
async fn smoke_ping() {
    let app = make_app();

    let req = Request::builder().uri("/ping").body(Body::empty()).unwrap();

    // use ServiceExt::oneshot on the Router
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), 200);

    // optionally test body content:
    let body_bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
    assert_eq!(&body_bytes[..], b"pong");
}
