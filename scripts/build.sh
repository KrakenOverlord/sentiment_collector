# For building on the EC2 server.

cargo build --release
cp target/release/sentiment_collector .
sudo systemctl restart sentiment_collector.service