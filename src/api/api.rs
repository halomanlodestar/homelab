use axum::routing::get;
use axum::Router;
use super::controllers::get_files_from;

pub async fn listen() {
  let app: Router = Router::new()
    .route("/files", get( get_files_from ));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

  axum::serve(listener, app).await.unwrap();
}