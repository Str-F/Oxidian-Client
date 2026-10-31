use crate::server::Server;
use clap::Parser;
use log::info;

mod client;
mod network;
mod protocol;
mod server;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "localhost")]
    host: String,

    #[arg(long, default_value_t = 25565)]
    port: u16,

    #[arg(long, default_value = "OxidianClientUsr")]
    username: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut client = client::Client::new(args.username);
    info!("=== Oxidian Client started ===");
    let server = Server::new(&args.host, args.port);
    client.run(&server).await;
}
