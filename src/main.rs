use log::info;

mod client;
mod network;
mod protocol;
mod server;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut client = client::Client::new();
    info!("=== Oxidian Client started ===");
    client.run().await;
}
