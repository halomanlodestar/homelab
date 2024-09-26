pub mod controllers;
pub mod router;

use router::Router;

pub async fn listen() {
    Router::initialize().await;
}
