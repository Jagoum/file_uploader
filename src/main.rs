mod db_config;
mod upload;
use axum::{
    Router,
    response::Html,
    routing::{get, post},
};
use tokio::net::TcpListener;
use upload::upload::file_uplaod;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/home", get(index))
        .route("/upload", post(file_uplaod));
    let addr = "0.0.0.0:7879";
    let socket = TcpListener::bind(addr).await.expect("Failed to bind addr");
    let server = axum::serve(socket, app);
    println!("Serving on http://{addr}");
    server.await.unwrap()
}

async fn index() -> Html<String> {
    Html(std::include_str!("../index.html").to_string())
}
