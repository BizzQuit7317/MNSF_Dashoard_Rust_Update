/*
    Here is the structure and implementation for the client object so it can be used more openly in other scripts
*/
use dotenv;
use mongodb::{bson::{doc, Document}, Client, Database};
use std::env;
use futures_util::TryStreamExt;
use chrono;
use serde_json::{json, Value};
use mongodb::bson;

pub struct ClientStruct {
    pub URI:String,
    pub DATABASE_STR:String,
    pub CLIENT:Client,
    pub DATABASE:Database,
    pub COLLECTION:Option<mongodb::Collection<mongodb::bson::Document>>,
}

impl ClientStruct {
    pub async fn new(database: &str) -> ClientStruct {
        let _ = dotenv::from_filename("C:/Users/Administrator/Rust Dashboard Rewrite/mongo_connect/config.env");
        let uri = env::var("MONGO_URI").unwrap();

        let client = Client::with_uri_str(&uri).await.expect("Error connecting to client");

        let db_obj = client.database(&database);

        ClientStruct {
            URI:uri,
            DATABASE_STR:database.to_string(),
            CLIENT:client,
            DATABASE:db_obj,
            COLLECTION:None,
        }
    }

    pub async fn get_collections(&self) -> Result<Vec<String>, mongodb::error::Error>{
        let collections = self.DATABASE.list_collection_names().await;
        collections
    }

    pub async fn select_collection(&mut self, collection_name:String) {
        let collection = self.DATABASE.collection::<mongodb::bson::Document>(&collection_name);
        self.COLLECTION = Some(collection);
    }

    pub async fn get_collection_data_full(&self) {
        match &self.COLLECTION {
            Some(collection) => {
                match collection.find(doc! {"Date": "2025-07-20"}).await {
                    Ok(mut cursor) => {
                        while let Some(result) = cursor.try_next().await.unwrap_or_else(|e| {
                            eprintln!("[ERROR] Error reading from cursor: {}", e);
                            None
                        }) {
                            // `result` is already the Document
                            println!("{:?}", result);
                        }
                    }
                    Err(e) => {
                        eprintln!("[ERROR] MongoDB find failed: {}", e);
                    }
                }
            }
            None => println!("[WARN] No collection selected! Did you call select_collection()?"),
        }
    }

    pub async fn get_collection_data(&self, filter: Document) {
        match &self.COLLECTION {
            Some(collection) => {    
                match collection.find_one(filter).await { 
                    Ok(Some(doc)) => {
                        println!("{:?}", doc);
                    }
                    Ok(None) => {
                        println!("[DEBUG] No documents found in the collection!");
                    }
                    Err(e) => {
                        println!("[ERROR] Failed to fetch document: {}", e);
                    }
                }
            }
            None => println!("[WARN] No collection selected! Did you call select_collection()?"),
        }
    }

    pub async fn push_document_collection(&self, document: Document) {
        match &self.COLLECTION {
            Some(collection) => {
                match collection.insert_one(document).await {
                    Ok(_) => {
                        println!("Document written to collection successfully!");
                    }
                    Err(e) => {
                        println!("[ERROR] Failed to write to document: {}", e);
                    }
                }
            }
            None => println!("[WARN] No collection selected! Did you call select_collection()?"),
        }
    }

    pub async fn push_multi_document_collection(&self, documents: Vec<Document>) {
        match &self.COLLECTION {
            Some(collection) => {
                match collection.insert_many(documents).await {
                    Ok(_) => {
                        println!("Documents written to collection successfully!");
                    }
                    Err(e) => {
                        println!("[ERROR] Failed to write to documenst: {}", e);
                    }
                }
            }
            None => println!("[WARN] No collection selected! Did you call select_collection()?"),
        }
    }

    pub fn value_to_documents(&self, value: Value) -> Vec<Document> {
        let mut final_result: Vec<Document> = Vec::new();
        let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();

        match value {
            Value::Array(arr) => {
                for obj in arr {
                    let bson_value = bson::to_bson(&obj);
                    match bson_value {
                        Ok(bson_doc) => {
                            match bson_doc.as_document() {
                                Some(doc) => {
                                    let mut no_ref_doc = doc.clone();
                                    no_ref_doc.insert("Stored_DateTime", &ts);
                                    final_result.push(no_ref_doc);
                                },
                                _ => println!("[DBG] Somethings happening"),
                            }
                        },
                        Err(e) => println!("[ERR] No data to add {}", e),
                    }
                }
            },
            Value::Object(_) => println!("[DBG] single object"),
            _ => println!("[ERR] Data not in correct format!"),
        }
        final_result
    }
}
