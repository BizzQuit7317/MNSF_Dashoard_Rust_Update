use rpassword;
use secrecy::{SecretString, ExposeSecret};
use std::fs::File;
use pgp::packet::{PacketParser, Packet};
use pgp::errors::Error;
use std::io::BufReader;
use pgp::crypto;
use pgp::types::StringToKey;
use rand::rngs::OsRng;

fn main() {
    println!("Enter passkey: ");
    let password = SecretString::new(rpassword::read_password().unwrap().into());

    let gpg_file = File::open("/home/ubuntu/rust_tests/gpg_broker/binkey.json.gpg").expect("[ERR]GPG file not found! ");

    let buffer = BufReader::new(gpg_file);

    let mut parser = PacketParser::new(buffer);

    //let mut session_key: &[u8] = &[];
    let mut session_key: Vec<u8> = Vec::new();
    let mut sym_algo = pgp::crypto::sym::SymmetricKeyAlgorithm::AES256;
    //let iterator = parser.next();

    //while let Some(ref val) = iterator {
    while let Some(ref val) = parser.next() {
        match val {
            Ok(packet) => {
                //println!("Packet -> {:?}", packet);
                match packet {
                    Packet::SymKeyEncryptedSessionKey(skesk) => {
                        //println!("raw key -> {:?}", &password.expose_secret().as_bytes());
                        //println!("skesk -> {:?}", skesk);

                        //let cunt = StringToKey::new_iterated();

                        let s2k = &skesk.s2k();
                        let algo = skesk.sym_algorithm();
                        let key_size = 32;

                        let derived_key = s2k.unwrap().derive_key(&password.expose_secret().as_bytes(), key_size).expect("[ERR]Failied to derive key! ");
                        session_key = derived_key.as_ref().to_vec();
                        //println!("s2k -> {:?}\nderived key -> {:?}\nderived key bytes -> {:?}\nalgo -> {:?}\nkey size -> {:?}", s2k, derived_key, derived_key.as_ref(), algo, key_size);

                        //match skesk.decrypt(&password.expose_secret().as_bytes()) {
                        //println!("session key field -> {:?}", skesk.encrypted_key());

                        match skesk.encrypted_key() {
                            Some(bytes) if bytes.is_empty() => {
                                println!("No bytes here apparently");
                            }
                            Some(bytes) => {
                                match skesk.decrypt(derived_key.as_ref()) {
                                    Ok(sk) => {
                                        println!("sk -> {:?}", sk);
                                    }
                                    Err(e) => println!("[ERR]Could not decrypt packet! {:?}", e),
                                }
                            }
                            None => println!("[ERR]No SKESK sent!"),
                        }

                        //match skesk.decrypt(derived_key.as_ref()) {
                        //    Ok(sk) => {
                        //        println!("sk -> {:?}", sk);
                        //    }
                        //    Err(e) => println!("[ERR]Could not decrypt packet! {:?}", e),
                        //}
                    }
                    Packet::SymEncryptedProtectedData(sepd) => {
                        //println!("Session key -> {:?}\ndata to decrypt -> {:?}, packet -> {:?}", session_key, std::any::type_name_of_val(&sepd), sepd.data());
                        let data_bytes = sepd.decrypt(&session_key, Some(sym_algo));
                        //let data_plain_text = String::from_utf8(data_bytes.unwrap()).expect("[ERR]Invalid UTF-8! ");
                        println!("data bytess -> {:?}", data_bytes);


                    }
                    _ => println!("[DBG] Val was a different type of packet!\nval -> {}", std::any::type_name_of_val(&val)),
                }
            }
            Err(e) => println!("[ERR]Parsing packet! {:?}", e),
        }
    }

}
