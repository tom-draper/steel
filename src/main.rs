use axum::extract::Path;
use axum::routing::get_service;
use axum::{
    response::{Html, IntoResponse, Json},
    routing::get, // Corrected import for the get handler
    Router,
};
use std::net::SocketAddr;
use tokio::fs;
use tower_http::services::ServeDir;

async fn serve_website() -> impl IntoResponse {
    println!("Requested index.html");
    let content = fs::read_to_string("frontend/build/index.html")
        .await
        .unwrap_or_else(|_| String::new());
    Html(content)
}

async fn serve_path(Path(path): Path<String>) -> axum::response::Response {
    println!("Requested path: {}", path);

    match fs::metadata(&path).await {
        Ok(metadata) => {
            if metadata.is_file() {
                println!("The path is a file.");
                return serve_file(Path(path)).await.into_response();
            } else if metadata.is_dir() {
                println!("The path is a directory.");
                return serve_directory(Path(path)).await.into_response();
            } else {
                println!("The path is neither a regular file nor a directory.");
                return (
                    axum::http::StatusCode::NOT_FOUND,
                    Json(String::from("Path not found")),
                )
                    .into_response();
            }
        }
        Err(e) => {
            println!("Failed to access path: {}", e);
            return serve_file(Path(path)).await.into_response();
        }
    }
}

async fn serve_path_root() -> axum::response::Response {
    println!("Requested root path");
    serve_path(Path(String::from("."))).await
}

async fn serve_file(Path(path): Path<String>) -> impl IntoResponse {
    println!("Requested file path: {}", path);
    match fs::read_to_string(path).await {
        Ok(content) => {
            println!("Content: {}", content);
            (axum::http::StatusCode::OK, Json(content))
        }
        Err(_) => {
            println!("File not found");
            (
                axum::http::StatusCode::NOT_FOUND,
                Json(String::from("File not found")),
            )
        }
    }
}

async fn serve_directory(Path(path): Path<String>) -> impl IntoResponse {
    println!("Requested directory path: {}", path);
    match fs::read_dir(path).await {
        Ok(mut dir) => {
            let mut content = Vec::new();
            while let Some(entry) = dir.next_entry().await.unwrap() {
                let path = entry.path();
                let path_str = path.to_str().unwrap().to_string();
                content.push(path_str);
            }
            println!("Content: {:?}", content);
            (axum::http::StatusCode::OK, Json(content))
        }
        Err(_) => {
            println!("Directory not found");
            (axum::http::StatusCode::NOT_FOUND, Json(vec![]))
        }
    }
}

async fn serve_map() -> impl IntoResponse {
    // crawl all directories to build a list of all files
    let mut content = Vec::new();
    let mut stack = vec![String::from(".")];
    while let Some(path) = stack.pop() {
        match fs::metadata(&path).await {
            Ok(metadata) => {
                if metadata.is_file() {
                    content.push(path[2..].to_string());
                } else if metadata.is_dir() {
                    let mut dir = fs::read_dir(path).await.unwrap();
                    while let Some(entry) = dir.next_entry().await.unwrap() {
                        let entry_path = entry.path();
                        let entry_path_str = entry_path.to_str().unwrap().to_string();
                        stack.push(entry_path_str);
                    }
                }
            }
            Err(_) => {
                println!("Failed to access path: {}", path);
            }
        }
    }
    println!("Content: {:?}", content);
    (axum::http::StatusCode::OK, Json(content))
}

#[tokio::main]
async fn main() {
    // Build the Axum application with a single route
    let app = Router::new()
        .nest(
            "/_path",
            Router::new()
                .route("/", get(serve_path_root))
                .route("/*path", get(serve_path)),
        )
        .nest(
            "/_file",
            Router::new()
                .route("/", get(serve_file))
                .route("/*path", get(serve_file)),
        )
        .nest(
            "/_directory",
            Router::new()
                .route("/", get(serve_directory))
                .route("/*path", get(serve_directory)),
        )
        .route(
            "/_map",
            get(serve_map)
            
        )
        .nest(
            "/_app",
            Router::new().route(
                "/*file",
                get_service(ServeDir::new("frontend/build/_app")).handle_error(
                    |error| async move {
                        (
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled error: {}", error),
                        )
                    },
                ),
            ),
        )
        .route("/", get(serve_website))
        .route("/*path", get(serve_website));

    // Define the server address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // Start the Axum server
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
