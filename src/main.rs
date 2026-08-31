mod client;
mod network;
mod protocol;
mod server;

use client::Client;

#[tokio::main]
async fn main() {
    let mut client = client::Client::new();
    println!("=== Oxidian Client started ===");
    client.run().await;
}
