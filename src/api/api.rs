use super::controllers::get_files::get_files_controller;
use super::controllers::read_file::read_file_controller;
use axum::routing::get;
use axum::Router;
use tower_http::cors::CorsLayer;

pub async fn listen() {
    let app: Router = Router::new()
        .route("/files", get(get_files_controller))
        .layer(CorsLayer::permissive())
        .route("/file", get(read_file_controller));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
