use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KeyData {
    pub id: String,
    pub key: String,
    pub secret: String,
    pub pass: String,
    pub account: String
}
