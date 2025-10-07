# Optional swap
sudo fallocate -l 4G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile

# Limit codegen units and parallel jobs
export RUSTFLAGS="-C codegen-units=1"
cargo build --release -j 1

# After compilation
sudo swapoff /swapfile
sudo rm /swapfile
