use tokio::net::UnixListener;
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

    fs::set_permissions(socket_path, fs::permission::from_mode(0o700)).expect("[ERR]Failed to set the file permissions!!! ");

    println!("Server listening on {}", socket_path);

    loop {
        let (stream, _addr) = listener.accept().await.expect("[ERR]Failed to pickup client!!! ");
        println!("[DBG]Client connected!!! "); //Do something more than just send a connected message and close
        drop(stream)
    }

    Ok(())
}
