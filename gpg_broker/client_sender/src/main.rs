mod structs;

use tokio::net::UnixStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use serde_json;

pub const AUTH_TOKEN: &str = "SUPER-DUPER-SECRET!!!!";
pub const TMP_EXCHANGE_ID: &str = "binance";

#[tokio::main]
async  fn main() {
    let socket_path = "/tmp/uds_learning.sock";

    let mut stream = UnixStream::connect(socket_path).await.expect("[ERR]Could not connect!");

    stream.write_all(AUTH_TOKEN.as_bytes()).await.unwrap();
    stream.write_all(b"\n").await.unwrap(); //delimiter

    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await.unwrap();
    let reply = String::from_utf8_lossy(&buf[..n]);

    if reply.trim() != "AUTH_OK" {
        println!("Authentication failed: {}", reply);
        return;
    }

    println!("Authenticated with server!");

    stream.write_all(TMP_EXCHANGE_ID.as_bytes()).await.unwrap();
    stream.write_all(b"\n").await.unwrap(); //delimiter

    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await.unwrap();
    let reply = String::from_utf8_lossy(&buf[..n]);

    match serde_json::from_str::<structs::raw_data>(&reply) {
        Ok(data) => {
            println!("{:?}", data);
        }
        Err(e) => println!("[ERR]Deserialising string! {:?}", e),
    }

}
