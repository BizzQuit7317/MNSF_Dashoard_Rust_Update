mod structs;
mod response_structs;

use mongo_connect::ClientStruct;
use mongodb::bson::doc;
use reqwest::Method;

#[tokio::main]
async fn main() {
    let Harmoney_Client = structs::HarmonyClient::new(String::from(""));
    let Harmoney_Balance = Harmoney_Client.get_total_balance().await;

    println!("Total harmoney ONE balance: {}", Harmoney_Balance);

    /*
    let mut MongoClient = ClientStruct::new("Raw_Exchange_Data").await;
    let _ = MongoClient.select_collection(String::from("test")).await;
    let _ = MongoClient.push_document_collection(doc! {"Date":"2025-08-20", "Exchange":""}).await;
    */
}