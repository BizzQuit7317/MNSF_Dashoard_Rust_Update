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

    pub async fn send_request<T: for<'de> Deserialize<'de> + Default>(
        &self,
        endpoint: &str,
        method: Method,
        body_data: Option<Value>,
        query_string: &str,
    ) -> Result<T, Error> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;
        let recv_window: u64 = 5000;

        // payload = raw queryString for GET/DELETE, raw JSON body string for POST/PUT
        let payload = match method {
            Method::GET | Method::DELETE => query_string.to_string(),
            _ => body_data
                .as_ref()
                .map(|b| serde_json::to_string(b).unwrap_or_default())
                .unwrap_or_default(),
        };

        // Official Bybit rule: NO separators, NO sorting - just concatenate
        let param_str = format!("{}{}{}{}", timestamp, self.API_KEY, recv_window, payload);
        let signature = self.get_signature(&param_str);

        let full_url = if query_string.is_empty() {
            format!("{}{}", self.BASE_URL, endpoint)
        } else {
            format!("{}{}?{}", self.BASE_URL, endpoint, query_string)
        };

        let mut headers = HeaderMap::new();
        headers.insert("X-BAPI-API-KEY", HeaderValue::from_str(&self.API_KEY).unwrap());
        headers.insert("X-BAPI-SIGN", HeaderValue::from_str(&signature).unwrap());
        headers.insert("X-BAPI-TIMESTAMP", HeaderValue::from_str(&timestamp.to_string()).unwrap());
        headers.insert("X-BAPI-RECV-WINDOW", HeaderValue::from_str(&recv_window.to_string()).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::new();
        let request_builder = match method {
            Method::GET => client.get(&full_url),
            Method::POST => client.post(&full_url).json(&body_data.unwrap_or_else(|| json!({}))),
            Method::PUT => client.put(&full_url).json(&body_data.unwrap_or_else(|| json!({}))),
            Method::DELETE => client.delete(&full_url),
            _ => return Ok(T::default()),
        };

        let response = request_builder.headers(headers).send().await?.json::<T>().await?;
        Ok(response)
    }

    pub async fn get_data(&mut self) {
        let wallet_balance = self.send_request::<Value>("/v5/account/wallet-balance", Method::GET, None, "accountType=UNIFIED").await;
        let coins_balance = self.send_request::<Value>("/v5/asset/transfer/query-account-coins-balance", Method::GET, None, "accountType=FUND").await;
        let positions = self.send_request::<Value>("/v5/position/list", Method::GET, None, "category=linear&settleCoin=USDT").await;
        let funding_fees = self.send_request::<Value>("/v5/account/transaction-log", Method::GET, None, "accountType=UNIFIED&category=linear&currency=USDT&type=SETTLEMENT").await;

        println!("#################################################\n{:?}\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n{:?}\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n{:?}\n{:?}\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n#################################################", wallet_balance, coins_balance, positions, funding_fees)
    }


}
