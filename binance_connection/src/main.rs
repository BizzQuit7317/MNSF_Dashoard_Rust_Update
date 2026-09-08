mod structs;

use serde_json::json;
use structs::BinanceClient;
use reqwest::Method;
use serde_json::Value;
use mongodb::{bson::{doc, Document}, Client, Database};

use std::fs::File;
use std::io::{self, BufReader, Read, Write};

use age::secrecy::SecretString;

#[tokio::main]
async fn main() {
    print!("KeyFile: ");
    io::stdout().flush().expect("Could not flush stdout!");

    let mut input_path_buffer = String::new();
    io::stdin().read_line(&mut input_path_buffer).expect("Could not read line!");

    let input_path = format!("{}.txt.age", input_path_buffer.trim());
    let passphrase = SecretString::from(
        rpassword::prompt_password("Passphrase: ").expect("Could not read passphrase!")
    );

    let ciphertext = BufReader::new(File::open(&input_path).expect("Could not find file!"));

    let decryptor = age::Decryptor::new_buffered(ciphertext).expect("Not a valid age file!");

    let identity = age::scrypt::Identity::new(passphrase);
    let mut reader = decryptor
        .decrypt(std::iter::once(&identity as &dyn age::Identity))
        .expect("Could not decrypt — wrong passphrase?");

    let mut plaintext = Vec::new();
    reader.read_to_end(&mut plaintext).expect("Could not read plaintext!");

    // If it's text
    let text = String::from_utf8(plaintext.clone()).expect("Not valid UTF-8!");
    let text_lines: Vec<&str> = text.split("\n").collect();

    let mut Binance_Client = structs::BinanceClient::new(text_lines[0].to_string(), text_lines[1].to_string()); //main

    let account = input_path.strip_suffix(".txt.age").and_then(|s| s.rsplit_once('_')).map(|(_, name)| name);

    Binance_Client.get_data(account.unwrap().to_string()).await;
}
