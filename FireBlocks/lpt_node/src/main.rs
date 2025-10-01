mod structs;
mod response_structs;

use mongo_connect::ClientStruct;
use mongodb::bson::doc;
use reqwest::Method;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let LTP_Client = structs::LPTClient::new(String::from(""));
    let LPT_Balance = LTP_Client.send_request::<Value>("", Method::GET, None).await;

    println!("LPT Balance -> {}", LPT_Balance);

    /*
    let mut MongoClient = ClientStruct::new("Raw_Exchange_Data").await;
    let _ = MongoClient.select_collection(String::from("test")).await;
    let _ = MongoClient.push_document_collection(doc! {"Date":"2025-08-20", "Exchange":"LPT"}).await;
    */
}