use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct raw_data {
    pub data: Vec<key_dat>
}

#[derive(Debug, Deserialize)]
pub struct key_data {
    pub id: String,
    pub key: String,
    pub secret: String,
    pub pass:  String,
    pub account: String
}
