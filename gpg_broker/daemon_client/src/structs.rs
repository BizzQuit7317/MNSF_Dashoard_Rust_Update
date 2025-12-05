use serde::Deserialize;
use tokio::net::UnixStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use serde_json;

#[derive(Debug, Deserialize)]
pub struct KeyData {
    pub id: String,
    pub key: String,
    pub secret: String,
    pub pass: String,
    pub account: String
}

impl Default for KeyData {
    fn default() -> Self {
        KeyData {
            id: "".to_string(),
            key: "".to_string(),
            secret: "".to_string(),
            pass: "".to_string(),
            account: "".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Client {
    pub AUTH_TOKEN: String,
    pub EXCHANGE_ID: String,
    pub SOCKET_PATH: String,
}

impl Client {
    pub fn new(auth_token: &str, exchange_id: &str) -> Client {
        Client {
            AUTH_TOKEN: String::from(auth_token),
            EXCHANGE_ID: String::from(exchange_id),
            SOCKET_PATH: String::from("/tmp/uds_learning.sock")
        }
    }

    pub async fn collectData(&self) ->  KeyData {
        let mut stream = UnixStream::connect(&self.SOCKET_PATH).await.expect("[ERR]Couldn't connect to the socket on path! ");

        stream.write_all(&self.AUTH_TOKEN.as_bytes()).await.unwrap();
        stream.write_all(b"\n").await.unwrap(); //delimiter

        let mut buf = vec![0u8; 1024];
        let n = stream.read(&mut buf).await.unwrap();
        let reply = String::from_utf8_lossy(&buf[..n]);

        if reply.trim() != "AUTH_OK" {
            println!("Authentication failed: {}", reply);
            return KeyData::default();
        }

        stream.write_all(&self.EXCHANGE_ID.as_bytes()).await.unwrap();
        stream.write_all(b"\n").await.unwrap(); //delimiter

        let mut buf = vec![0u8; 1024];
        let n = stream.read(&mut buf).await.unwrap();
        let reply = String::from_utf8_lossy(&buf[..n]);



        let mut key_data = KeyData::default();

        match serde_json::from_str::<Vec<KeyData>>(&reply) {
            Ok(mut data) => {
                if !data.is_empty() {
                    key_data = data.remove(0); // take the first element
                }
                //key_data = data;
            }
            Err(e) => println!("[ERR]Deserialising string! {:?}", e),
        }

        key_data
    }
}
