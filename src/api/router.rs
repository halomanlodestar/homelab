use axum::{routing::get, Router as AxumRouter};
use tower_http::cors::CorsLayer;

use super::controllers::{get_files::get_files_controller, read_file::read_file_controller};

pub struct Router;

impl Router {
    pub async fn initialize() {
        let app: AxumRouter = AxumRouter::new()
            .route("/files", get(get_files_controller))
            .layer(CorsLayer::permissive())
            .route("/file", get(read_file_controller));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

        axum::serve(listener, app).await.unwrap();
    }
}
