use reqwest::{Client, Error, Method};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::response_structs;

pub struct HarmonyClient {
    pub ADDRESS: String,
    pub BASE_URL: String,
}

impl HarmonyClient {
    pub fn new (address: String) -> HarmonyClient {
        HarmonyClient {
            ADDRESS: address,
            BASE_URL: String::from("https://api.s0.t.hmny.io")
        }
    }

    pub async fn send_request<T: for<'de> Deserialize<'de> + Default>(&self, endpoint: &str, method: Method, body_data: Option<Value>, address_at_end: bool) -> Result<T, Error> {
        //The base variable being set to true will use BASE_URL set to false will use AVAX_URL
        let mut full_url = String::new();
        match address_at_end {
            true => {
                full_url = format!("{}{}{}", self.BASE_URL, endpoint, self.ADDRESS);
            },
            false => {
                full_url = format!("{}{}", self.BASE_URL, endpoint);
            }
        }

        //let full_url = format!("{}{}", self.BASE_URL, endpoint);
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

    pub async fn get_total_balance(&self) -> f64 {
        let mut total_balance: f64 = 0.0; //Buffer to hold final balance result

        let available_balance = self.send_request::<response_structs::AvailableResponse>("", Method::POST, Some(json!({
            "id":"1",
            "jsonrpc":"2.0",
            "method":"hmyv2_getBalance",
            "params":["one1fvvceuv6lzd0574858ct6w7qv6lp6g4x675zg0"]
        })), false).await;
        total_balance += available_balance.unwrap().result as f64;

        let delegated_balance = self.send_request::<response_structs::DelegatedResponse>("", Method::POST, Some(json!({
            "id":"1",
            "jsonrpc":"2.0",
            "method":"hmy_getDelegationsByDelegator",
            "params":["one1fvvceuv6lzd0574858ct6w7qv6lp6g4x675zg0"]
        })), false).await;

        for result in delegated_balance.unwrap().result {
            total_balance += result.amount as f64;
        }
        
        total_balance
    }
}