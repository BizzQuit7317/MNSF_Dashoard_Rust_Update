use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use anyhow::Result;
use std::process::Command;
use rpassword;
use secrecy::{SecretString, ExposeSecret};

pub const AUTH_TOKEN: &str = "SUPER-DUPER-SECRET!!!!";

#[tokio::main]
async fn main() -> Result<()> {
    println!("Enter passsword: ");

    let PASSWORD = SecretString::new(rpassword::read_password().unwrap().into());

    let mut AUTH_STATE: bool = false;

    let socket_path = "/tmp/uds_learning.sock"; //This file will be created whenthe script is run

    if Path::new(socket_path).exists() {
        fs::remove_file(socket_path).expect("[ERR]Could not remove file!!! "); //Mkae sure the file doesnt already exist
    }

    let listener = UnixListener::bind(socket_path).expect("[ERR]Failed to bind to socket!!! ");

    fs::set_permissions(socket_path, fs::Permissions::from_mode(0o700)).expect("[ERR]Failed to set the file permissions!!! ");

    println!("Server listening on {}", socket_path);

    loop {
        let (stream, _addr) = listener.accept().await.expect("[ERR]Failed to pickup client!!! ");
        //println!("[DBG]Client connected!!! ");
        let value = PASSWORD.clone();
        tokio::spawn(async move {
            handle_client(stream, AUTH_STATE, &value.expose_secret()).await;
        });

        //drop(stream)
    }

    Ok(())
}


async fn handle_client(mut stream: UnixStream, mut auth_state: bool, pass: &str) {
    let mut buf = vec![0u8; 1024];

    loop {
        match stream.read(&mut buf).await {
            Ok(0) => {
                println!("[DBG] Client disconnected");
                return;
            }
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buf[..n]).trim().to_string();;
                println!("[DBG] Received: {}\nauth state -> {}", msg, auth_state);

                if msg  == AUTH_TOKEN && auth_state == false{
                    stream.write_all(b"AUTH_OK").await.unwrap();
                    auth_state = true;
                } else if msg != AUTH_TOKEN && auth_state == true{
                    let output  = Command::new("/home/ubuntu/rust_tests/gpg_broker/secret_daemon/target/release/secret_daemon").arg(pass).output().expect("[ERR]Running decryptor! ");
                    let mut stdout_string = String::new();
                    if output.status.success() {
                        stdout_string = String::from_utf8_lossy(&output.stdout).to_string();
                        println!("output ->  {}", stdout_string);
                    }
                    stream.write_all(stdout_string.as_bytes()).await.unwrap();
                    auth_state = false;
                } else  {
                    stream.write_all(b"AUTH_FAIL").await.unwrap();
                }

                println!("[DBG] Finished handling  current message above\n##############################################");

                /*
                // Send a response back to the client
                let reply = format!("AUTH_OK");
                if let Err(e) = stream.write_all(reply.as_bytes()).await {
                    eprintln!("[ERR] Failed to send response: {}", e);
                    return;
                }
                */

            }
            Err(e) => {
                eprintln!("[ERR] Reading client! {}", e);
                return
            }
        }
    }
}
