use std::path::PathBuf;

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

    #[arg(long, env = "OXIDIAN_ASSETS")]
    assets: Option<PathBuf>,
}

fn find_assets_dir(explicit: Option<PathBuf>) -> PathBuf {
    if let Some(path) = explicit {
        return path;
    }

    let mut candidates = vec![PathBuf::from("assets")];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("assets"));
        }
    }
    candidates
        .into_iter()
        .find(|path| path.join("data").is_dir())
        .unwrap_or_else(|| PathBuf::from("assets"))
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let assets_dir = find_assets_dir(args.assets);
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut client = client::Client::new(args.username, assets_dir);
    info!("=== Oxidian Client started ===");
    let server = Server::new(&args.host, args.port);
    client.run(&server).await;
}
