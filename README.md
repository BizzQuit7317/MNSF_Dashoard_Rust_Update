# MNSF_Dashoard_Rust_Update
Same as the MNSF_Dashboard repo but backend code updated in rust and updated app structure

# To Do
- Update token between sender andd client to be proper tokens instead of strings
- Need to setup a encrypted mongodb table to store user data (Ussername, Pasword  and Token)
- write a function  to check each token against one from the db, it should load in the fully encrypted db when launched and every so often should check for new entries or other changes
- Deserialise decrypted keys from sender_client
- Make sender_client a structure so it can be used by the exchange scripts
- Connect each exchange script to the sender structure to read the keys withut having to see them in plain text

# Important
- If compiling code on a smaller machine like a ec2 t2.micro use the safe_compile.sh script to add a 4gb swap and limit compilation to 1 crate at a time, this will massivly increase compile time but stop the server from crashing. Just **add the safe_compile.sh script to the same dir as the Cargo.toml** files and run it from there
- Make sure to **run safely compiled scripts from /target/release/**
- If any issue compiling the secret daemon script you may need to ensure gpg dev tools are installed
```
sudo apt update
sudo apt install -y pkg-config libgpgme-dev libgpg-error-dev libassuan-dev
```
- See dir UDS_Test to see the breakdown of how the unix domain socket will work for the gpg broker

# Key security
- gpg encrypted keys in json files
- each key pair gets its own encrypted json
- write a secret daemon process (some script to work as an inbetween)
- each exchange with interact with the secret daemon via Unix domain socket
- this way we only need to enter the secret passphrase, which not stored on the server to start the application

# How the secret daemon works
- There are 3 structures
- The decryptor struct to read the keys
- The server which is the daemon and liten on the uni socket
- The client which goes in each exchange call to interact with the socket
- the socket is given a pasword by the user when it start up, this password is used by the decryptor to decrypt the keys
- the client also has a unique token to authenticate with the socket so it doesnt just give keys to any client
- the client then sendds the keys into the exchnage request

# Setting up keys
- Create the plain text json file in this format
```
[
    {
        "id": "binance",
        "key": "",
        "secret": "",
        "pass": "",
        "account":"0"
    },
    {
        "id": "binance",
        "key": "",
        "secret": "",
        "pass": "",
        "account":"1"
    }
]
```
- Then run
```
gpg --symmetric --cipher-algo AES256 -o <key>.json.gpg <key>.json
shred -u <key>.json
```

- If you need to check passwords or change anything you candecrypt the file manually with
```
gpg --output <key>.json --decrypt <key>.json.gpg

```

# Changes
- No longer 1 control script, each exchange independantly pushes raw data to DB
- All raw data form exchanges are permiated in DB so we can always recover or repair
- Everything should be modular, easier to update individual exchange or add new ones, also remvoves risk of 1 error creating a chain effect and breaking the entire app

# Data Flow
- Read from exchange, saving raw calls to DB
- Sanatise and format exchange data from DB
- Each exchange struct should run endpoint call asynchronously (ensure request limits to not overload the server)
- Use systemd to run the polling scripts for each exchange (with restart when stopped)
