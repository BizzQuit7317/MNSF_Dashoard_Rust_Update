mod structs;
mod response_structs;

use mongo_connect::ClientStruct;
use mongodb::bson::doc;
use serde_json::json;
use reqwest::Method;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let mut Eth_Client = structs::EthClient::new(String::from(""));
    let Eth_Balance = Eth_Client.get_total_balance().await;

    println!("Eth balance -> {:?}", Eth_Balance);

    /*
    let mut MongoClient = ClientStruct::new("Raw_Exchange_Data").await;
    let _ = MongoClient.select_collection(String::from("test")).await;
    let _ = MongoClient.push_document_collection(doc! {"Date":"2025-08-13", "Exchange":"eth"}).await;
    */
}
