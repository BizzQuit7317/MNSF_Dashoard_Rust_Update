mod structs;

use tokio::net::UnixStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use serde_json;

#[tokio::main]
async fn run_client() {
    println!("[MAIN] starting!!!");
    let client = structs::Client::new("SUPER-DUPER-SECRET!!!!", "binance");
    println!("[MAIN] Client created");
    let _ = client.collectData().await;
    println!("[MAIN] Finsihed running!!!");
}
