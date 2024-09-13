mod api;
mod files;

use api::api::listen;

#[tokio::main]
async fn main() {
    listen().await;
}
