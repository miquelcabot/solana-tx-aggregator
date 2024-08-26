mod aggregator;

use aggregator::SolanaAggregator;
use std::net::SocketAddr;

use clap::Parser;
use url::Url;
use warp::Filter;

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
    init_tracing();
    let args = Args::parse();

    let rpc_url = args.rpc_url.parse::<Url>().expect("Invalid RPC URL");
    let local_address = args
        .local_address
        .parse::<SocketAddr>()
        .expect("Invalid local address");

    tracing::info!("🌐 Solana RPC URL: {}", rpc_url);
    tracing::info!("🛠️ Local RESTful API address: {}", local_address);

    // Create a new Solana aggregator
    let mut aggregator = SolanaAggregator::new(rpc_url.as_str());
    tokio::spawn(async move {
        aggregator.fetch_transactions().await;
    });

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(local_address).await;
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

