mod structs;

use serde_json::json;
use structs::BinanceClient;
use reqwest::Method;
use serde_json::Value;
use mongodb::{bson::{doc, Document}, Client, Database};

#[tokio::main]
async fn main() {
    let mut Binance_Client = structs::BinanceClient::new("Ooj4sM3xp2sgDODrd4JvC0BeqZXcQyGLkZpjn5cSqegnIN4wUr0SsJLrFG2bInIy".to_string(), "Kfloz0VcZVsEgNHFWxs8n1gAVJwKvbNaAoFw0c42w1I1TJivN7Urq7A6oPZ7E9Cm".to_string()); //main
    Binance_Client.get_data().await;

    let mut Binance_Client_sub = structs::BinanceClient::new("pbVDqkOk5FAZpBbtIIV4B3oILx4WWZGWNRgUKeAvHbDn7UXFjBWT5eJ92toQIc9c".to_string(), "JSZqTHXOmeyoVekENbae8z2m4p9uX3YINTTgnW96MUrlbMhGg4zfNyhhJN62Ia2o".to_string()); //sub 1
    Binance_Client_sub.get_data().await;

    let mut Binance_Client_sub2 = structs::BinanceClient::new("opk1ON0Dtv0CPu0LTxU6W6cOlCSsL1Lmss6n0uQ8WUiCBCt666SPDbsLsuk8GjGb".to_string(), "GcsDsdKVOm20s2phNxgGkVk1HdemfgcKdbjzdJVK8ltwfDsIjgypxhkskCGTnaTK".to_string()); //sub 2
    Binance_Client_sub2.get_data().await;

    let mut Binance_Client_sub3 = structs::BinanceClient::new("TrHLFkhph1cSwVw3ZtZFUIJgadR4BLRb6vBIPMm3BWoEjihWZe4ecf4fSSb9j287".to_string(), "lutsan9yMJPL9sBUZgMdJ4x3A4nsYrwQarvRiQoPNqaQtDCdDUHaZwqGc94knlca".to_string()); //sub 3
    Binance_Client_sub3.get_data().await;

    let mut Binance_Client_sub4 = structs::BinanceClient::new("MwRfATCSA1nbGzjABxBUVs3uKC3QA5ZHrAgLHrZeO03L82maWmR5PIE1QktpEps4".to_string(), "rqLNxevmiLBVLu31eS7iytoDT7P47bX0wyWbspXo70ADqQzLIGJvlTAmKF6AtCtq".to_string()); //sub 4
    Binance_Client_sub4.get_data().await;

    //Push data into db
    let client = mongodb::Client::with_uri_str("mongodb://localhost:27017".to_string()).await.unwrap();
    let db = client.database("TEST_ENV");
    let test_collection: mongodb::Collection<Document> = db.collection("test_docs_binance");

    let test_doc = doc! { "test": "Success" };
    let insert_result = test_collection.insert_one(test_doc).await.unwrap();
    println!("Complete!");
}
