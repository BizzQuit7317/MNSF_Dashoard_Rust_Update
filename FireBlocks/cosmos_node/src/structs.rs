use std::collections::HashMap;
use reqwest::{Method, Error, Client};
use serde_json::{Value, json};
use serde::Deserialize;

use crate::response_structs;

pub struct CosmosClient {
    pub ADDRESS: String,
    pub BASE_URL: String,
    pub VALOPER_ADDRESS: Option<String>
}

impl CosmosClient {
    pub fn new(address: String, valoper_address: Option<String>) -> CosmosClient {
        let base_url_maps: HashMap<_, _> = [
            ("cosmos","https://cosmos-lcd.quickapi.com:443"),
            ("secret","https://secret-4.api.trivium.network:1317"),
            ("kava","https://api.data.kava.io"),
        ].into_iter().collect();

        let identifier = address.split_once("1").map(|(identifier, _)| identifier).unwrap_or(&address);     

        //println!("{:?}", base_url_maps.get(identifier).unwrap());

        let mut base_url = String::from(*base_url_maps.get(identifier).unwrap());

        CosmosClient {
            ADDRESS: address,
            BASE_URL: base_url,
            VALOPER_ADDRESS: valoper_address
        }
    }

    pub async fn send_request<T: for<'de> Deserialize<'de> + Default>(&mut self, endpoint: &str, method: Method, body_data: Option<Value>, address_at_end: bool) -> Result<T, Error> {
        let mut url = String::new();
        if address_at_end {
            url = format!("{}{}{}", self.BASE_URL, endpoint, self.ADDRESS);
        } else {
            url = format!("{}{}", self.BASE_URL, endpoint);
        }
        
        let client = Client::new();

        let request_builder = match method {
            Method::GET => client.get(&url),
            Method::POST => client.post(&url).json(&body_data.unwrap_or_else(|| json!({}))),
            Method::PUT => client.put(&url).json(&body_data.unwrap_or_else(|| json!({}))),
            Method::DELETE => client.delete(&url),
            _ => {
                return Ok(T::default())
            }
        };

        let response = request_builder.send().await?.json::<T>().await?;

        Ok(response) 
    }

    pub async fn get_total_balance(&mut self) -> f64{
        println!("[DBG] Starting total balance function...");
        let delagator_balance = &self.send_request::<response_structs::DelegationResponse>("/cosmos/staking/v1beta1/delegations/", Method::GET, None, true).await;
        let mut delegator_value:f64 = 0.0;
        match delagator_balance {
            Ok(delagator_response) => {
                match &delagator_response.delegation_responses {
                    Some(delagated_outputs) if !delagated_outputs.is_empty() => {
                        for outputs in delagated_outputs {
                            if outputs.balance.denom == String::from("uscrt") || outputs.balance.denom == String::from("uatom") {
                                delegator_value += outputs.balance.amount.parse::<f64>().unwrap();
                            }
                        }
                    }
                    _ => println!("[WRN] No delagator outputs")
                }
            }
            Err(e) => println!("[ERR] getting delagted {}", e)
        }
        
        let available_balance = &self.send_request::<response_structs::AvailableResponse>("/cosmos/bank/v1beta1/balances/", Method::GET, None, true).await;
        let mut available_value: f64 = 0.0;
        match available_balance {
            Ok(available_response) => {
                match &available_response.balances {
                    Some(available_outputs) if !available_outputs.is_empty() => {
                        for outputs in available_outputs {
                            available_value += outputs.amount.parse::<f64>().unwrap();
                        }
                    }
                    _ => println!("[WRN] No available outputs")
                }
            }
            Err(e) => println!("[ERR] getting available {}", e)
        }

        let rewards_endpoint = format!("/cosmos/distribution/v1beta1/delegators/{}/rewards", self.ADDRESS);
        let rewards_balance = &self.send_request::<response_structs::NoImplResponse>(&rewards_endpoint, Method::GET, None, false).await;
        let rewards_value: f64 = 0.0;

        let unbonding_endpoint = format!("/cosmos/staking/v1beta1/delegators/{}/unbonding_delegations", self.ADDRESS);
        let unbonding_balance = &self.send_request::<response_structs::NoImplResponse>(&unbonding_endpoint, Method::GET, None, false).await;
        let unbonding_value: f64 = 0.0;

        let mut total: f64 = 0.0;
        match &self.VALOPER_ADDRESS {
            Some(address) => {
                let commision_endpoint = format!("/cosmos/distribution/v1beta1/validators/{}/commission", address);
                let commision_balance = &self.send_request::<Value>(&commision_endpoint, Method::GET, None, false).await;
            }
            None => {
                println!("[WRN] No valoper address was given");
                total = delegator_value + available_value + rewards_value  + unbonding_value;
            }
        }

        total
        
    }
}