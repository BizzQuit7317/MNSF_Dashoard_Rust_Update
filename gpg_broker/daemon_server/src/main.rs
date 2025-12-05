mod structs;
mod lib;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    let server = structs::Server::new();
    let _ = Arc::new(server).start().await;
}
