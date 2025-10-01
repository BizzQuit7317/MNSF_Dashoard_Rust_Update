use reqwest::{Client, Error, Method};
use serde_json::{json, Value};
use std::time::Duration;
use serde::Deserialize;

pub struct GenericClient {
    ADDRESS:String,
    BASE_URL:String,
}

impl GenericClient {
    pub fn new(address:String, base_url: String) -> GenericClient {
        GenericClient {
            ADDRESS:address,
            BASE_URL: base_url
        }
    }

    pub async fn send_request<T: for<'de> Deserialize<'de> + Default>(&self, endpoint: &str, method: Method, body_data: Option<Value>, address_at_end: bool, timeout: u64) -> Result<T, Error> {
        let mut full_url = String::new();
        match address_at_end {
            true => {full_url = format!("{}{}{}", self.BASE_URL, endpoint, self.ADDRESS)}
            false => {full_url = format!("{}{}", self.BASE_URL, endpoint)}
        }
        println!("{}", full_url);
        let client = Client::new();
        let request_builder = match method {
            Method::GET => client.get(&full_url),
            Method::POST => client.post(&full_url).json(&body_data.unwrap_or_else(|| json!({}))),
            Method::PUT => client.put(&full_url).json(&body_data.unwrap_or_else(|| json!({}))),
            Method::DELETE => client.delete(&full_url),
            _ => {
                return Ok(T::default())
            }
        }.timeout(Duration::from_secs(timeout));

        let response = request_builder.send().await?.json::<T>().await?;

        Ok(response)
    }
}