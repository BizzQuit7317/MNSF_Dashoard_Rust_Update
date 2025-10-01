use serde::Deserialize;
use serde_json::{json, Value};
use tokio_tungstenite::connect_async;
use futures_util::{SinkExt, StreamExt};
use url::Url;
use bs58;
use twox_hash::XxHash64;
use std::hash::Hasher;
use std::vec;
use blake2::{Blake2b, Digest};
use blake2::Blake2bVar;
use hex::{self, FromHex};
use parity_scale_codec::{Decode, Error};


use crate::response_structs;

pub struct PolkadotClient {
    ADDRESS: String,
    BASE_URL: String,
}

impl PolkadotClient {
    pub fn new(address: String) -> PolkadotClient {
        PolkadotClient {
            ADDRESS: address,
            BASE_URL: String::from("wss://kusama-rpc.polkadot.io")
        }
    }

    pub fn twox128(&self, input: &str) -> [u8; 16] {
        let mut h1 = XxHash64::with_seed(0); //0 is defined by the documentation
        h1.write(input.as_bytes());
        let part1 = h1.finish();

        let mut h2 = XxHash64::with_seed(1); //1 is defined by the documentation
        h2.write(input.as_bytes());
        let part2 = h2.finish();

        let mut result = [0u8; 16];
        result[..8].copy_from_slice(&part1.to_le_bytes());
        result[8..].copy_from_slice(&part2.to_le_bytes());

        result
    }

    pub fn blake2_128_concat(&self, data: &[u8]) -> Vec<u8> {
        use blake2::digest::{Update, VariableOutput};

        let mut hasher = blake2::Blake2bVar::new(16).unwrap(); //16 for a 16 byte output
        hasher.update(data);

        /*
        let mut hash = vec![0u8; 16];
        hasher.finalize_variable(&mut hash).expect("Hash finalize failed");
        [hash, data.to_vec()].concat()
        */

        let mut hash = vec![0u8; 16];
        hasher.finalize_variable(&mut hash).unwrap();

        let mut result = Vec::with_capacity(16 + data.len());
        result.extend_from_slice(&hash);
        result.extend_from_slice(data); // exactly original input
        result
    }

    pub fn encode_address(&self, moduel: &str, storage: &str) -> String {
        let mut address_bytes = bs58::decode(&self.ADDRESS).into_vec().expect("[ERR] Invalid ss58 address!");
        address_bytes = address_bytes[1..33].to_vec(); //1..33 is a standard 32 bytes values, 0 digit network prefix, 34/35 digits checksum
        //println!("[DBG] decode -> {:?}", address_bytes);

        let mut key = Vec::new();
        key.extend_from_slice(&self.twox128(moduel));
        key.extend_from_slice(&self.twox128(storage));
        key.extend_from_slice(&self.blake2_128_concat(&address_bytes));
        
        let hex_key = format!("0x{}", hex::encode(key)); //need to manally add 0x for json-rpc request
        hex_key
    }

    pub async fn websocket_connect<T: for<'de> Deserialize<'de> + Default>(&self, method: &str, params: Vec<serde_json::Value>) -> Result<T, Box<dyn std::error::Error>> {
        //println!("[DBG] Starting websocket connection");
        let url = Url::parse(&self.BASE_URL).unwrap();

        let (ws_stream, _) = connect_async(url).await.expect("[ERR] Failed to connect to websocket! ");
        //println!("[DBG] Connected to websocket");

        let (mut write, mut read) = ws_stream.split();

        //let encoded_address = self.encode_address();
        //println!("[DBG] encoded_address -> {}", encoded_address);

        //Need to find how this is spposed too look
        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params, //[&encoded_address, null],
            "id": 2
        });

        write.send(tokio_tungstenite::tungstenite::Message::Text(request.to_string())).await.expect("[ERR] Failed to send message! ");
        //println!("[DBG] Message sent");

        while let Some(msg) = read.next().await {
            match msg {
                Ok(tokio_tungstenite::tungstenite::Message::Text(txt)) => {
                    //println!("[DBG] Response -> {}", txt);
                    let resp: T = serde_json::from_str(&txt)?;
                    return Ok(resp);
                }
                Ok(tokio_tungstenite::tungstenite::Message::Ping(_)) => {
                    println!("[DBG] Ping from server (ignored, pong handled internally)");
                }
                Ok(other) => {
                    println!("[DBG] Response (non txt) -> {:?}", other);
                }
                Err(e) => {
                    eprintln!("[ERR] Failed to read response: {}", e);
                    break;
                }
            }
        }

        Ok(T::default())
    }

    pub async fn get_total_balance(&self) -> f64 {
        let mut total_balance: f64 = 0.0;

        let encoded_address = self.encode_address("System", "Account");

        let response = self.websocket_connect::<response_structs::EncodedResponse>("state_getStorageAt", vec![json!(encoded_address), serde_json::Value::Null]).await;
        let response_result_option = response.unwrap().result;
        match response_result_option {
            Some(response_result) => {
                let response_str = &response_result[2..];
                let response_bytes = Vec::from_hex(response_str).unwrap();

                let account_data = response_structs::AccountBytes::decode(&mut &response_bytes[..]);
                total_balance += account_data.unwrap().data.free as f64;
            },
            null => println!("[ERR] No result found! ")
        }

        let staking_encoded_address = self.encode_address("Staking", "Ledger");

        let staking_response = self.websocket_connect::<response_structs::EncodedResponse>("state_getStorageAt", vec![json!(staking_encoded_address), serde_json::Value::Null]).await;
        let staking_response_result_option = staking_response.unwrap().result;
        match staking_response_result_option {
            Some(staking_response_result) => {
                //println!("raw response -> {}", staking_response_result);
                let staking_response_str = &staking_response_result[2..];
                let staking_response_bytes = Vec::from_hex(staking_response_str).unwrap();

                let staking_account_data = response_structs::AccountBytes::decode(&mut &staking_response_bytes[..]);
                //println!("staking account data -> {:?}", staking_account_data);
                total_balance += staking_account_data.unwrap().data.free as f64; 
            },
            null => println!("[ERR] No result found in staking! ")
        }

        total_balance
    }
}