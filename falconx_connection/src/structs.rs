use reqwest::{Method, Error, Client, header::{HeaderMap, HeaderValue, CONTENT_TYPE}};
use std::time::{SystemTime, UNIX_EPOCH};
use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use serde_json::{Value, json};
use hex::encode;
use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;

type HmacSha256 = Hmac<Sha256>;

pub struct FalconxClient {
    pub API_KEY:String,
    pub API_SECRET:String,
    pub PASSPHRASE:String,
    pub BASE_URL:String
}

impl FalconxClient {
    pub fn new(api_key: String, api_secret:String, passphrase:String) -> FalconxClient {
        FalconxClient {
            API_KEY: api_key,
            API_SECRET: api_secret,
            PASSPHRASE: passphrase,
            BASE_URL: String::from("https://api.falconx.io/v1/api/native-custody")
        }
    }

    pub fn authenticate(&self, message: String) -> String {
        let key_bytes = general_purpose::STANDARD.decode(self.API_SECRET.trim()).expect("[ERR] decoding secret key");
        let mut mac = HmacSha256::new_from_slice(&key_bytes).expect("[ERR] converting secret key!");
        let _ = mac.update(message.trim().as_bytes());
        let result = mac.finalize();
        general_purpose::STANDARD.encode(result.into_bytes())
    }

    pub async fn send_request<T: for<'de> Deserialize<'de> + Default>(&self, endpoint: &str, method: Method, body_data: Option<Value>) -> Result<T, Error> {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let timestamp = since_the_epoch.as_secs();
        let TimeStamp = timestamp as u64;

        let body = body_data.as_ref().map(|b| b.to_string()).unwrap_or_else(|| String::new());
        let message = format!("{}{}{}{}", TimeStamp.to_string(), method, endpoint, body);

        let signature = self.authenticate(message);

        let mut headers = HeaderMap::new();
        headers.insert("FX-ACCESS-KEY", HeaderValue::from_str(&self.API_KEY).unwrap());
        headers.insert("FX-ACCESS-PASSPHRASE", HeaderValue::from_str(&self.PASSPHRASE).unwrap());
        headers.insert("FX-ACCESS-SIGN", HeaderValue::from_str(&signature).unwrap());
        headers.insert("FX-ACCESS-TIMESTAMP", HeaderValue::from_str(&TimeStamp.to_string()).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let full_url = format!("{}{}", self.BASE_URL, endpoint);
        let client = Client::new();

        let request_builder = match method {
            Method::GET => client.get(&full_url),
            Method::POST => client.post(&full_url).json(&body_data.unwrap_or_else(|| json!({}))),
            Method::PUT => client.put(&full_url).json(&body_data.unwrap_or_else(|| json!({}))),
            Method::DELETE => client.delete(&full_url),
            _ => {
                return Ok(T::default())
            }
        };

        let response = request_builder.send().await?.json::<T>().await?;

        Ok(response)
    }
}