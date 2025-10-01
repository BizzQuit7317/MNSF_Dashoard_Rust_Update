mod structs;

use mongo_connect::ClientStruct;
use mongodb::bson::doc;
use reqwest::Method;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let Bybit_Client = structs::BybitClient::new(String::from(""), String::from(""));
    let response = Bybit_Client.send_request::<Value>("/v5/account/wallet-balance", Method::GET, None).await;

    println!("{:?}", response);

    /*
    let mut MongoClient = ClientStruct::new("Raw_Exchange_Data").await;
    let _ = MongoClient.select_collection(String::from("test")).await;
    let _ = MongoClient.push_document_collection(doc! {"Date":"2025-07-28", "Exchange":"bybit"}).await;
    */
}