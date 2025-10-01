mod structs;
mod response_structs;

use mongo_connect::ClientStruct;
use mongodb::bson::doc;
use reqwest::Method;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let Cartesi_Client = structs::CartesiClient::new(String::from(""), String::from(""));
    let response = Cartesi_Client.send_request::<Value>("endpoint", Method::GET, None).await;

    println!("response total balance -> {:?}", response.unwrap().total_balance);
    /*    
    let mut MongoClient = ClientStruct::new("Raw_Exchange_Data").await;
    let _ = MongoClient.select_collection(String::from("test")).await;
    let _ = MongoClient.push_document_collection(doc! {"Date":"2025-07-29", "Exchange":"cartesi"}).await;
    */
}