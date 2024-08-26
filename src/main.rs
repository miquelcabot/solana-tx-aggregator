use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use anyhow::Context;
use clap::Parser;
use solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use solana_transaction_status::UiTransactionEncoding;
use tokio::time::sleep;
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

    println!("RPC URL: {}", rpc_url);
    println!("Local address: {}", local_address);

    collect_data(&rpc_url).await?;

    Ok(())
}

async fn collect_data(rpc_url: &Url) -> Result<(), Error> {
    // Create a new RPC client connected to the a Solana RPC URL
    let client = RpcClient::new(rpc_url.to_string());

    // Create a vector to store transactions
    let mut transactions = Vec::new();

    // Set up the parameters for fetching signatures
    let mut last_signature: Option<Signature> = None;

    loop {
        // Fetch the latest signatures for confirmed transactions
        let signature_results = client.get_signatures_for_address_with_config(
            &client
                .get_account_with_commitment(
                    &client.get_identity().unwrap(),
                    CommitmentConfig::finalized(),
                )
                .unwrap()
                .value
                .unwrap()
                .owner,
            GetConfirmedSignaturesForAddress2Config {
                before: None,
                until: last_signature,
                limit: Some(1000),
                commitment: Some(CommitmentConfig::finalized()),
            },
        );

        match signature_results {
            Ok(signatures) => {
                // Process each transaction signature
                for signature_info in signatures.iter() {
                    let signature = Signature::from_str(&signature_info.signature)
                        .expect("Invalid signature format");
                    println!("Signature: {}", signature);
                    match client.get_transaction(&signature, UiTransactionEncoding::JsonParsed) {
                        Ok(transaction) => {
                            tracing::info!(
                                "📄 Storing transaction with signature {} ({})",
                                signature,
                                transaction.block_time.unwrap()
                            );
                            transactions.push(transaction);
                            last_signature = Some(signature);
                        }
                        Err(err) => {
                            tracing::error!("❌ Error fetching transaction: {}", err);
                        }
                    }
                }
            }
            Err(err) => {
                tracing::error!("❌ Error fetching signatures: {}", err);
            }
        }

        // Wait for a bit before fetching new transactions
        sleep(Duration::from_secs(1)).await;
    }

    // Ok(())
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
