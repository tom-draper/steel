use axum::{
    response::{Html, IntoResponse},
    routing::get, // Corrected import for the get handler
    Router,
};
use std::net::SocketAddr;
use tokio::fs;
use tower_http::services::ServeDir;
use axum::routing::get_service;

async fn serve_index() -> impl IntoResponse {
    let content = fs::read_to_string("frontend/build/index.html")
        .await
        .unwrap_or_else(|_| String::from("<h1>Something went wrong!</h1>"));
    Html(content)
}

#[tokio::main]
async fn main() {
    // Build the Axum application with a single route
    let app = Router::new().route("/", get(serve_index)).nest(
        "/_app",
        Router::new().route(
            "/*file",
            get_service(ServeDir::new("frontend/build/_app")).handle_error(|error| async move {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled error: {}", error),
                )
            }),
        ),
    );

    // Define the server address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // Start the Axum server
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
