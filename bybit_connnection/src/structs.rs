use reqwest::{Client, Error, Method, header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT}};
use std::time::{SystemTime, UNIX_EPOCH};
use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use hex::encode;
use tungstenite::{connect, Message};
use url::Url;
use serde_json::{json, Value};
use serde::Deserialize;

pub struct BybitClient {
    pub API_KEY:String,
    pub API_SECRET:String,
    pub BASE_URL:String,
}

impl BybitClient {
    pub fn new(api_key: String, api_secret: String) -> BybitClient {
        //Uses the default bybit base, this method should be used unless you need to change the base
        BybitClient {
            API_KEY: api_key,
            API_SECRET: api_secret,
            BASE_URL: String::from("https://api.bybit.com"), //Default base url
        }
    }

    pub fn get_signature(&self, base_signature: &str) -> String {
        let key_bytes = self.API_SECRET.trim().as_bytes();
        let msg_bytes = base_signature.trim().as_bytes(); 

        type HmacSha256 = Hmac<Sha256>; 
        let mut mac = HmacSha256::new_from_slice(key_bytes).expect("Error with hmac: "); 
        mac.update(msg_bytes); 
        let result = mac.finalize(); 
        encode(result.into_bytes()) 
    }

    pub async fn send_request<T: for<'de> Deserialize<'de> + Default>(&self, endpoint: &str, method: Method, body_data: Option<Value>) -> Result<T, Error> {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let timestamp = since_epoch.as_millis();
        let TimeStamp = timestamp as u64;

        let base_sign = format!("accountType=UNIFIED&api_key={}&recv_window=5000&timestamp={}", self.API_KEY, TimeStamp);
        let signature = self.get_signature(&base_sign);
        let full_sign = format!("{}&sign={}", base_sign, signature);

        let full_url = format!("{}{}?{}", self.BASE_URL, endpoint, full_sign);
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