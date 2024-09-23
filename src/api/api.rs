use crate::api::router::Router;

pub async fn listen() {
    Router::initialize().await;
}
