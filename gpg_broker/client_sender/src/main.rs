use tokio::net::UnixStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async  fn main() {
    let socket_path = "/tmp/uds_learning.sock";

    let mut stream = UnixStream::connect(socket_path).await.expect("[ERR]Could not connect!");

    let test_message = "Salam Allahkum brother!";

    stream.write_all(test_message.as_bytes()).await.unwrap();
    stream.write_all(b"\n").await.unwrap(); //delimiter
}
