use reqwest::{Client, Error, Method, header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT}};
use std::time::{SystemTime, UNIX_EPOCH};
use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use hex::encode;
use serde_json::{json, Value};
use url::form_urlencoded;
use std::collections::HashMap;
use serde::Deserialize;

pub struct BinanceClient {
    pub API_KEY:String,
    pub API_SECRET:String,
    pub BASE_URL:String
}

impl BinanceClient {
    pub fn new(api_key: String, api_secret: String) -> BinanceClient {
        BinanceClient {
            API_KEY:api_key,
            API_SECRET:api_secret,
            BASE_URL:String::from("https://api.binance.com")
        }
    }

    pub fn get_signature(&self, query_string: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.API_SECRET.as_bytes()).expect("Error creating hamc!");
        let _ = mac.update(query_string.as_bytes());
        let result = mac.finalize();
        encode(result.into_bytes())
    }

    pub async fn send_request<T: for<'de> Deserialize<'de> + Default>(&mut self, base_endpoint: &str, endpoint: &str, method: Method, body_data: Option<Value>) -> Result<T, Error> {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let timestamp = since_epoch.as_millis();
        let TimeStamp = timestamp as u64;

        self.BASE_URL = format!("https://{}.binance.com", base_endpoint);

        let params: HashMap<String, String> = match &body_data {
            Some(data) => {
                serde_json::from_value(data.clone()).unwrap_or_default()
            }
            None => HashMap::new(),
        };

        let mut query_string = form_urlencoded::Serializer::new(String::new()).extend_pairs(params.iter()).finish();
        query_string = format!("{}&timestamp={}", query_string, TimeStamp);
        let signature = self.get_signature(&query_string);
        let url = format!("{}{}?{}&signature={}", self.BASE_URL, endpoint, query_string, signature);

        let mut headers = HeaderMap::new();
        let _ = headers.insert("X-MBX-APIKEY", HeaderValue::from_str(&self.API_KEY).unwrap());

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

        Ok(response)       

    }
}