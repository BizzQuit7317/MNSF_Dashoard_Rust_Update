use serde::Deserialize;
use reqwest::{Client, Error, Method};
use serde_json::{json, Value};
use ethers::prelude::*;
use std::sync::Arc;
use crate::response_structs;

pub struct LPTClient {
    pub ADDRESS: String,
    pub TOKEN_CONTRACT: String,
    pub STAKING_CONTRACT: String,
    pub BASE_URL: String,
}

impl LPTClient {
    pub fn new(address: String) -> LPTClient {
        LPTClient {
            ADDRESS: address,
            TOKEN_CONTRACT: String::from(""),
            STAKING_CONTRACT: String::from(""),
            BASE_URL: String::from("https://arb1.arbitrum.io/rpc"),
        }
    }

    pub async fn send_request<T: for<'de> Deserialize<'de> + Default>(&self, endpoint: &str, method: Method, body_data: Option<Value>) -> f64 {
        let mut total_balance: f64 = 0.0;

        let provider = Provider::<Http>::try_from(&self.BASE_URL).expect("Could not instantiate HTTP provider");
        let provider = Arc::new(provider);

        abigen!(
            Token,
            r#""#,
            event_derives(serde::Deserialize, serde::Serialize)
        );
        
        abigen!(
            Staking,
            r#""#,
            event_derives(serde::Deserialize, serde::Serialize)
        );

        let token_address: Address = self.TOKEN_CONTRACT.parse().expect("Error parsing token address");
        let token_contract = Token::new(token_address, provider.clone());

        let staking_address: Address = self.STAKING_CONTRACT.parse().expect("Error parsing staking address");
        let staking_contract = Staking::new(staking_address, provider.clone());

        let user_address: Address = self.ADDRESS.parse().expect("Error parsing address");

        let _balance = token_contract.balance_of(user_address).call().await;
        match _balance {
            Ok(balance) => {
                println!("balance -> {:?}", balance);
                total_balance += balance.as_u128() as f64;
            },
            Err(e) => println!("[ERR] No balance retrived {}", e),
        }

        let _pending_staked = staking_contract.pending_stake(user_address, U256::from(0)).call().await;
        match _pending_staked {
            Ok(pending_staked) => {
                println!("staked -> {:?}", pending_staked);
                total_balance += pending_staked.as_u128() as f64;
            },
            Err(e) => println!("[ERR] No staked retrived {}", e),
        }

        /*MIGHT NEED LATER IF WEE NEED TO USE BONDED_AMOUNT FOR ANYTHING 
        let _staked = staking_contract.get_delegator(user_address).call().await;
        match _staked {
            Ok(staked) => {
                println!("staked -> {:?}", staked);
            },
            Err(e) => println!("[ERR] No staked retrived {}", e),
        }
        */

        total_balance
    }
}