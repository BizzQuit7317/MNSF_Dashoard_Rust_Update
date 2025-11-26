use tokio::net::UnixStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

pub const AUTH_TOKEN: &str = "SUPER-DUPER-SECRET!!!!";

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
}
