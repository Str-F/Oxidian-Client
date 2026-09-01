mod client;
mod network;
mod protocol;
mod server;

#[tokio::main]
async fn main() {
    let mut client = client::Client::new();
    println!("=== Oxidian Client started ===");
    client.run().await;
}
