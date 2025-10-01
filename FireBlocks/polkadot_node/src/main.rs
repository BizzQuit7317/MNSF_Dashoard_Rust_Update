mod structs;
mod response_structs;

use mongo_connect::ClientStruct;
use mongodb::bson::doc;
use reqwest::Method;

#[tokio::main]
async fn main() {
    let keys = ["", "", "", "", "", ""];
    
    for key in keys {
        let Polkadot_Client = structs::PolkadotClient::new(String::from(key));
        let polkadot_balance = Polkadot_Client.get_total_balance().await;

        println!("#######################\nKey: {}\nTotal Balance -> {}\n#######################\n", key, polkadot_balance);
    }
    
    /* 
    let mut MongoClient = ClientStruct::new("Raw_Exchange_Data").await;
    let _ = MongoClient.select_collection(String::from("test")).await;
    let _ = MongoClient.push_document_collection(doc! {"Date":"2025-08-29", "Exchange":"polkadot"}).await;
    */
}