mod structs;

use serde_json::json;
use structs::BinanceClient;
use reqwest::Method;
use serde_json::Value;
use mongodb::{bson::{doc, Document}, Client, Database};

#[tokio::main]
async fn main() {
    let mut Binance_Client = structs::BinanceClient::new("".to_string(), "".to_string()); //main
    Binance_Client.get_data().await;

    let mut Binance_Client_sub = structs::BinanceClient::new("".to_string(), "".to_string()); //sub 1
    Binance_Client_sub.get_data().await;

    let mut Binance_Client_sub2 = structs::BinanceClient::new("".to_string(), "".to_string()); //sub 2
    Binance_Client_sub2.get_data().await;

    let mut Binance_Client_sub3 = structs::BinanceClient::new("".to_string(), "".to_string()); //sub 3
    Binance_Client_sub3.get_data().await;

    let mut Binance_Client_sub4 = structs::BinanceClient::new("".to_string(), "".to_string()); //sub 4
    Binance_Client_sub4.get_data().await;

    //Push data into db
    let client = mongodb::Client::with_uri_str("mongodb://localhost:27017".to_string()).await.unwrap();
    let db = client.database("TEST_ENV");
    let test_collection: mongodb::Collection<Document> = db.collection("test_docs_binance");

    let test_doc = doc! { "test": "Success" };
    let insert_result = test_collection.insert_one(test_doc).await.unwrap();
    println!("Complete!");
}
