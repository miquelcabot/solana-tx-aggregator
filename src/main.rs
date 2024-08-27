mod aggregator;
mod api;
mod transaction_details;
mod utils;

use crate::aggregator::SolanaAggregator;
use crate::api::create_api;
use crate::transaction_details::TransactionDetails;

use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use clap::Parser;
use solana_sdk::signature::Signature;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The address of the RPC URL for Solana
    #[arg(short, long, default_value = "https://api.devnet.solana.com/")]
    rpc_url: String,

    /// The address of this local RESTful API server
    #[arg(short, long, default_value = "0.0.0.0:8080")]
    local_address: String,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    init_tracing();

    // Parse the command-line arguments
    let args = Args::parse();

    let rpc_url = args.rpc_url.parse::<Url>().expect("Invalid RPC URL");
    let local_address = args
        .local_address
        .parse::<SocketAddr>()
        .expect("Invalid local address");

    // Log the RPC URL and local address
    tracing::info!("🌐 Solana RPC URL: {}", rpc_url);
    tracing::info!("🛠️ Local RESTful API address: {}", local_address);

    // Create a new transactions hash map
    let transactions_hash: HashMap<Signature, TransactionDetails> = HashMap::new();
    let transactions = Arc::new(Mutex::new(transactions_hash));

    // Create a new Solana aggregator
    let aggregator = SolanaAggregator::new(rpc_url.as_str(), Arc::clone(&transactions));

    // Fetch transactions in the background
    tokio::spawn(async move {
        aggregator.fetch_transactions().await;
    });

    // Start the API server
    let api = create_api(transactions);
    warp::serve(api).run(local_address).await;
}

/// Initialize tracing
///
/// This function initializes the tracing subscriber.
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
