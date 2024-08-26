mod aggregator;

use aggregator::SolanaAggregator;
use std::net::SocketAddr;

use anyhow::Context;
use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The address of the RPC URL for Solana
    #[arg(short, long, default_value = "https://api.devnet.solana.com/")]
    rpc_url: String,

    /// The address of this local RESTful API server
    #[arg(short, long, default_value = "0.0.0.0:0")]
    local_address: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    let args = Args::parse();

    let rpc_url = args.rpc_url.parse::<Url>().context("Invalid RPC URL")?;
    let local_address = args
        .local_address
        .parse::<SocketAddr>()
        .context("Invalid local address")?;

    tracing::info!("🌐 Solana RPC URL: {}", rpc_url);
    tracing::info!("🛠️ Local RESTful API address: {}", local_address);

    // Create a new Solana aggregator
    let mut aggregator = SolanaAggregator::new(rpc_url.as_str());
    aggregator.fetch_transactions().await;

    Ok(())
}

pub fn init_tracing() {
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;

    let env = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .with_env_var("RUST_LOG")
        .from_env_lossy();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false);
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(env)
        .init();
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Connection failed: {0:?}")]
    ConnectionFailed(std::io::Error),
}
