use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
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

        tokio::spawn(async move {
            handle_client(stream).await;
        });

        //drop(stream)
    }

    Ok(())
}


async fn handle_client(mut stream: UnixStream) {
    let mut buf = vec![0u8; 1024];

    loop {
        match stream.read(&mut buf).await {
            Ok(0) => {
                println!("[DBG] Client disconnected");
                return;
            }
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buf[..n]);
                println!("[DBG] Received: {}", msg);

                // Send a response back to the client
                let reply = format!("Server got: {}", msg);
                if let Err(e) = stream.write_all(reply.as_bytes()).await {
                    eprintln!("[ERR] Failed to send response: {}", e);
                    return;
                }
            }
            Err(e) => {
                eprintln!("[ERR] Reading client! {}", e);
                return
            }
        }
    }
}
