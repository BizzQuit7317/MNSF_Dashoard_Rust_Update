use tokio::net::UnixListener;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let socket_path = "/tmp/uds_learning.sock"; //This file will be created whenthe script is run


}
