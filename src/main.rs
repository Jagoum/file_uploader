mod db_config;
mod upload;
use axum::{
    Router,
    response::Html,
    routing::{get, post},
};
use db_config::getfile::get_files;
use tokio::net::TcpListener;
use upload::upload::file_uplaod;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/upload", post(file_uplaod))
        .route("/compressed", get(get_files))
        .fallback("404 Page Not Found");
    let addr = "0.0.0.0:7879";
    let socket = TcpListener::bind(addr).await.expect("Failed to bind addr");
    let server = axum::serve(socket, app);
    println!("Serving on http://{addr}");
    server.await.unwrap()
}

async fn index() -> Html<String> {
    Html(std::include_str!("../index.html").to_string())
}
