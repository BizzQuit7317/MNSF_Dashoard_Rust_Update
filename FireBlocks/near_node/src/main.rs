mod structs;
mod response_structs;

use mongo_connect::ClientStruct;
use mongodb::bson::doc;
use reqwest::Method;

#[tokio::main]
async fn main() {
    let mut Near_Client = structs::NearClient::new(String::from(""));
    let _ = Near_Client.get_networks().await;
    let balance = Near_Client.get_total_balance().await;

    println!("Near balance -> {:?}", balance);

    /* 
    let mut MongoClient = ClientStruct::new("Raw_Exchange_Data").await;
    let _ = MongoClient.select_collection(String::from("test")).await;
    let _ = MongoClient.push_document_collection(doc! {"Date":"2025-08-20", "Exchange":"near"}).await;
    */
}