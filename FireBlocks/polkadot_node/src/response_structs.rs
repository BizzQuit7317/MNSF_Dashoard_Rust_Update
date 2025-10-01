use serde::{Serialize, Deserialize};
use serde_json::Value;
use parity_scale_codec::{Decode, Error};

fn default_null_string() -> String {
    "Null".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodedResponse {
    pub jsonrpc: String,
    pub id: u8,
    pub result: Option<String>,
}

impl Default for EncodedResponse {
    fn default() -> Self {
        EncodedResponse {
            jsonrpc: String::new(),
            id: 0,
            result: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Decode)]
pub struct AccountBytes {
    pub nonce: u32,
    pub consumers: u32,
    pub providers: u32,
    pub sufficients: u32,
    pub data: AccountDataBytes,
}

#[derive(Debug, Clone, Serialize, Deserialize, Decode)]
pub struct AccountDataBytes {
    pub free: u128,
    pub reserved: u128,
    pub frozen: u128,
    pub flags: u128,
}
