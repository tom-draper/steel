use axum::extract::Path;
use axum::routing::get_service;
use axum::{
    response::{Html, IntoResponse, Json},
    routing::get, // Corrected import for the get handler
    Router,
};
use serde::Serialize;
use std::net::SocketAddr;
use std::env;
use dotenv::dotenv;
use tokio::fs;
use tower_http::services::ServeDir;

async fn is_running_in_docker() -> bool {
    fs::metadata("/.dockerenv").await.is_ok()
}

async fn get_root_folder() -> String {
    if is_running_in_docker().await {
        String::from("/steel")
    } else {
        env::var("ROOT_FOLDER").unwrap_or(String::from("."))
    }
}

async fn serve_website() -> impl IntoResponse {
    let content = fs::read_to_string("frontend/build/index.html")
        .await
        .unwrap_or_else(|_| String::new());
    Html(content)
}

async fn serve_path(Path(path): Path<String>) -> axum::response::Response {
    match fs::metadata(&path).await {
        Ok(metadata) => {
            if metadata.is_file() {
                return serve_file(Path(path)).await.into_response();
            } else if metadata.is_dir() {
                return serve_directory(Path(path)).await.into_response();
            } else {
                return (
                    axum::http::StatusCode::NOT_FOUND,
                    Json(String::from("Path not found")),
                )
                    .into_response();
            }
        }
        Err(e) => {
            eprintln!("Failed to access path: {}", e);
            return serve_file(Path(path)).await.into_response();
        }
    }
}

async fn serve_path_root() -> axum::response::Response {
    let root_folder = get_root_folder().await;
    serve_path(Path(root_folder)).await
}

async fn serve_file(Path(path): Path<String>) -> impl IntoResponse {
    match fs::read_to_string(path).await {
        Ok(content) => (axum::http::StatusCode::OK, Json(content)),
        Err(_) => (
            axum::http::StatusCode::NOT_FOUND,
            Json(String::from("File not found")),
        ),
    }
}

#[derive(Debug, Serialize)]
struct Node {
    path: String,
    is_directory: bool,
}

async fn serve_directory(Path(path): Path<String>) -> impl IntoResponse {
    match fs::read_dir(path).await {
        Ok(mut dir) => {
            let mut content = Vec::new();
            while let Some(entry) = dir.next_entry().await.unwrap() {
                let path = entry.path();
                let path_str = path.to_str().unwrap().to_string();
                let is_directory = fs::metadata(&path).await.unwrap().is_dir();
                content.push(Node {
                    path: path_str,
                    is_directory,
                });
            }
            (axum::http::StatusCode::OK, Json(content)).into_response()
        }
        Err(_) => (axum::http::StatusCode::NOT_FOUND, Json::<Vec<Node>>(vec![])).into_response(),
    }
}

async fn serve_map() -> impl IntoResponse {
    let mut content = Vec::new();
    let root_folder = get_root_folder().await;
    let mut stack = vec![root_folder];
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
                eprintln!("Failed to access path: {}", path);
            }
        }
    }
    (axum::http::StatusCode::OK, Json(content))
}

async fn serve_map_bfs() -> impl IntoResponse {
    let mut content = Vec::new(); // Stores discovered file paths
    let root_folder = get_root_folder().await;
    let mut queue = vec![root_folder]; // BFS queue starts with the root path

    while let Some(path) = queue.pop() {
        match fs::metadata(&path).await {
            Ok(metadata) => {
                if metadata.is_file() {
                    // Safely strip "./" or use the full path
                    let relative_path = std::path::Path::new(&path)
                        .strip_prefix(".")
                        .unwrap_or(std::path::Path::new(&path));
                    content.push(relative_path.to_string_lossy().to_string());
                } else if metadata.is_dir() {
                    // Attempt to read the directory and enqueue its entries
                    match fs::read_dir(&path).await {
                        Ok(mut dir) => {
                            while let Ok(Some(entry)) = dir.next_entry().await {
                                if let Ok(entry_path) = entry.path().canonicalize() {
                                    if let Some(entry_path_str) = entry_path.to_str() {
                                        queue.insert(0, entry_path_str.to_string());
                                        // Enqueue at the front
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read directory '{}': {}", path, e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to access path '{}': {}", path, e);
            }
        }
    }

    // Log the discovered files and return as JSON
    (axum::http::StatusCode::OK, Json(content))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

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
        .route("/_map", get(serve_map))
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
