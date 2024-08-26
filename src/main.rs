use std::net::SocketAddr;

use anyhow::Context;
use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The address of the RPC URL for Solana
    #[arg(short, long, default_value = "https://api.devnet.solana.com/")]
    rpc_address: String,

    /// The address of this local RESTful API server
    #[arg(short, long, default_value = "0.0.0.0:0")]
    local_address: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let rpc_address = args.rpc_address.parse::<Url>().context("Invalid RPC URL")?;
    let local_address = args
        .local_address
        .parse::<SocketAddr>()
        .context("Invalid local address")?;

    println!("RPC URL: {}", rpc_address);
    println!("Local address: {}", local_address);

    collect_data(&rpc_address, &local_address).await?;

    Ok(())
}

async fn collect_data(rpc_address: &Url, local_address: &SocketAddr) -> Result<(), Error> {
    // Your code here
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Connection failed: {0:?}")]
    ConnectionFailed(std::io::Error),
}
