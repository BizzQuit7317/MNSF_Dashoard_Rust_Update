use reqwest::{Client, Error, Method, header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT}};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha512, Digest};
use hmac::{Hmac, Mac};
use hex;
use serde::Serialize;
use tungstenite::{connect, Message};
use url::Url;
use std::thread;
use serde_json::{json, Value};
use serde::Deserialize;

type HmacSha512 = Hmac<Sha512>;

pub struct GateClient {
    pub API_KEY:String,
    pub API_SECRET:String,
    pub PREFIX:String,  
    pub BASE_URL:String,
}

impl GateClient {
    pub fn new(api_key: String, api_secret: String) -> GateClient {
        GateClient {
            API_KEY: api_key,
            API_SECRET: api_secret,
            PREFIX:String::from("/api/v4"),
            BASE_URL:String::from("https://api.gateio.ws"),
        }
    }

    pub fn signature(&self, endpoint: &str, method: &str) -> (String, String, String) {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let timestamp = since_the_epoch.as_secs();
        let t = timestamp as u64;

        let mut hasher = Sha512::new();
        let input = b"";
        hasher.update(input);
        let result = hasher.finalize();
        let hashed_payload = hex::encode(result);

        let s = format!("{}\n{}\n{}\n{}\n{}", method, endpoint, "", hashed_payload, t);

        let mut mac = HmacSha512::new_from_slice(self.API_SECRET.as_bytes()).expect("Invalid key length");
        mac.update(s.as_bytes());
        let sign = hex::encode(mac.finalize().into_bytes());

        (self.API_KEY.clone(), t.to_string(), sign)
    }

    pub async fn send_request<T: for<'de> Deserialize<'de> + Default>(&self, endpoint: &str, method: Method, body_data: Option<Value>) -> Result<T, Error> {
        let full_endpoint = format!("{}{}", self.PREFIX, endpoint);
        let (api_key, timestamp, signature) = self.signature(&full_endpoint, &method.to_string());
        let url = format!("{}{}{}", self.BASE_URL, self.PREFIX, endpoint);
        let client = Client::new();

        let mut headers = HeaderMap::new();
        headers.insert("KEY", HeaderValue::from_str(&api_key).unwrap());
        headers.insert("Timestamp", HeaderValue::from_str(&timestamp).unwrap());
        headers.insert("SIGN", HeaderValue::from_str(&signature).unwrap());

        let request_builder = match method {
            Method::GET => client.get(&url).headers(headers),
            Method::POST => client.post(&url).headers(headers).json(&body_data.unwrap_or_else(|| json!({}))),
            Method::PUT => client.put(&url).headers(headers).json(&body_data.unwrap_or_else(|| json!({}))),
            Method::DELETE => client.delete(&url).headers(headers),
            _ => {
                return Ok(T::default())
            }
        };

        let response = request_builder.send().await?.json::<T>().await?;

        Ok(response)
    }
}