use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World!!!</strong>")})
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on port 8080 \n");
    axum::serve(listener, app).await.unwrap();
}
