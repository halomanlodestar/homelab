mod api;
mod disk;

use api::listen;

#[tokio::main]
async fn main() {
    listen().await;
}
