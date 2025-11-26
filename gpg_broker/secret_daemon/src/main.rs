use rpassword;
use secrecy::{SecretString, ExposeSecret};
use std::fs::File;
use pgp::packet::{PacketParser, Packet};
use std::io::BufReader;
use std::io::Read;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Password argument missing.");
        std::process::exit(1); // Exit with a non-zero code to signal failure
    }

    let password = SecretString::new(args[1].clone().into());//SecretString::new(rpassword::read_password().unwrap().into());

    let gpg_file = File::open("/home/ubuntu/rust_tests/gpg_broker/binkey.json.gpg").expect("[ERR]GPG file not found! ");

    let buffer = BufReader::new(gpg_file);

    let mut parser = PacketParser::new(buffer);

    let mut session_key: Vec<u8> = Vec::new();
    let sym_algo = pgp::crypto::sym::SymmetricKeyAlgorithm::AES256;

    while let Some(ref val) = parser.next() {
        match val {
            Ok(packet) => {
                match packet {
                    Packet::SymKeyEncryptedSessionKey(skesk) => {
                        let s2k = &skesk.s2k();
                        let key_size = 32; //from documentation specifying the byte rangefrom documentation specifying the byte range

                        let derived_key = s2k.unwrap().derive_key(&password.expose_secret().as_bytes(), key_size).expect("[ERR]Failied to derive key! ");
                        session_key = derived_key.as_ref().to_vec();

                        match skesk.encrypted_key() {
                            Some(bytes) if bytes.is_empty() => {
                                println!("No bytes here apparently");
                            }
                            Some(_bytes) => {
                                match skesk.decrypt(derived_key.as_ref()) {
                                    Ok(sk) => {
                                        println!("sk -> {:?}", sk);
                                    }
                                    Err(e) => println!("[ERR]Could not decrypt packet! {:?}", e),
                                }
                            }
                            None => println!("[ERR]No SKESK sent!"),
                        }

                    }
                    Packet::SymEncryptedProtectedData(sepd) => {
                        match sepd.decrypt(&session_key, Some(sym_algo)) {
                            Ok(data_bytes) => {

                                let mut inner_parser = PacketParser::new(&data_bytes[..]);

                                while let Some(inner_val) = inner_parser.next() {
                                    match inner_val {
                                        Ok(inner_packet) => {
                                            match inner_packet {
                                                Packet::LiteralData(literal_data) => {
                                                    let plaintext_bytes = literal_data.data();

                                                    match String::from_utf8(plaintext_bytes.to_vec()) {
                                                        Ok(plaintext) => {
                                                            println!("Plain Text:\n{}", plaintext);
                                                        }
                                                        Err(_) => {
                                                            println!("[ERR]Data is not valid UTF-8. Raw bytes: {:?}", plaintext_bytes);
                                                        }
                                                    }
                                                },
                                                Packet::CompressedData(comp_data) => {
                                                    match comp_data.decompress() {
                                                        Ok(mut decomp_data) => {
                                                            let mut decompressed_bytes = Vec::new();

                                                            match decomp_data.read_to_end(&mut decompressed_bytes) {
                                                                Ok(_) => {
                                                                    let mut deepest_parser = PacketParser::new(&decompressed_bytes[..]);

                                                                    while let Some(deep_val) = deepest_parser.next() {
                                                                        match deep_val {
                                                                            Ok(deep_packet) => {
                                                                                if let Packet::LiteralData(literal_data) = deep_packet {
                                                                                    let plaintext_bytes = literal_data.data();

                                                                                    match String::from_utf8(plaintext_bytes.to_vec()) {
                                                                                        Ok(plaintext) => {
                                                                                            println!("Plain Text:\n{}", plaintext);
                                                                                        }
                                                                                        Err(e) => {
                                                                                            println!("[ERR] Data is not valid UTF-8. Error: {:?}", e);
                                                                                            println!("Raw bytes: {:?}", plaintext_bytes);
                                                                                        }
                                                                                    }
                                                                                }

                                                                            }
                                                                            Err(e) => println!("[ERR]Parsing deepest packet! {:?}", e),
                                                                        }
                                                                    }
                                                                }
                                                                Err(e) => println!("[ERR]Failed to decompress {}", e),
                                                            }

                                                        }
                                                        Err(e) => println!("[ERR]No data! {:?}", e),
                                                    }
                                                },
                                                _ => println!("[DBG] Inner packet was: {}", std::any::type_name_of_val(&inner_packet)),
                                            }
                                        }
                                        Err(e) => println!("[ERR] Parsing inner packet! {:?}", e),
                                    }
                                }
                            }
                            Err(e) => println!("[ERR] Could not decrypt SymEncryptedProtectedData! {:?}", e),
                        }


                    }
                    _ => println!("[DBG] Val was a different type of packet!\nval -> {}", std::any::type_name_of_val(&val)),
                }
            }
            Err(e) => println!("[ERR]Parsing packet! {:?}", e),
        }
    }

}
