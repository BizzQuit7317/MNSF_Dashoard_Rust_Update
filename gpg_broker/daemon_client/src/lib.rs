mod structs;

use tokio::net::UnixStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use serde_json;

pub async fn run_client() {
    let client = structs::Client::new("SUPER-DUPER-SECRET!!!!", "binance");
    let _ = client.collectData().await;
}
