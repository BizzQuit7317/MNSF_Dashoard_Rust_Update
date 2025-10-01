use reqwest::{Client, Error, Method, header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT, HeaderName}};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use url::form_urlencoded;
use hmac::{Hmac, Mac};
use sha2::{Sha512, Sha256, Digest};
use base64::{encode, decode};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::borrow::Cow;
use serde::Deserialize;

pub struct KrakenClient {
    pub API_KEY:String,
    pub API_SECRET:String,
    pub BASE_URL:String,
}

impl KrakenClient {
    // Contructor method
    pub fn new(api_key: String, api_secret: String) -> KrakenClient {
        KrakenClient {
            API_KEY: api_key,
            API_SECRET: api_secret,
            BASE_URL: String::from("https://api.kraken.com"),
        }
    }

    pub fn build_message(&self, encoded: &[u8], endpoint: &str) -> Vec<u8> {
        let endpoint_bytes = endpoint.as_bytes(); //Convert the ednpoint slice to bytes

        let mut hasher = Sha256::new(); //Create the new hash object
        hasher.update(encoded); //Use the hash object on the encoded data
        let hash_result = hasher.finalize(); //End the hashing process

        let mut message = Vec::new(); //Create a vector to store data
        message.extend_from_slice(endpoint_bytes);
        message.extend_from_slice(&hash_result); //Add the data points

        message
    }

    pub fn get_signature(&self, data: &Option<Value>, endpoint: &str) -> String {

        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let timestamp = since_epoch.as_millis();
        let nonce = timestamp as u64;

        let mut postdata: String = format!("nonce={}", nonce);

        let encoded = postdata.as_bytes(); //Convert the form to bytes

        let msg = self.build_message(encoded, endpoint); //Build the message

        let decoded_secret = decode(&self.API_SECRET); 

        type HmacSha512 = Hmac<Sha512>;
        let mut mac = HmacSha512::new_from_slice(&decoded_secret.unwrap()).expect("Error with hmac");
        mac.update(&msg);

        let result = mac.finalize().into_bytes();

        encode(result)
    }

    pub async fn send_request<T: for<'de> Deserialize<'de> + Default>(&self, endpoint: &str, method: Method, body_data: Option<Value>) -> Result<T, Error> {
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        headers.insert("API-Key", HeaderValue::from_str(&self.API_KEY).unwrap());

        let signature = self.get_signature(&body_data, endpoint);
        headers.insert("API-Sign", HeaderValue::from_str(&signature).unwrap());

        let url = format!("{}{}", self.BASE_URL, endpoint);

        let client = Client::new();

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
        //let response = client.post(full_url).headers(headers).form(&data).send().await?.text().await?;

        Ok(response)
    }
}