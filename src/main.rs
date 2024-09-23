mod api;
mod disk;

use api::api::listen;

#[tokio::main]
async fn main() {
    listen().await;
}
