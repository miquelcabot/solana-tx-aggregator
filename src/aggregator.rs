use crate::transaction_details::TransactionDetails;
use crate::utils;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use solana_transaction_status::{EncodedTransaction, UiMessage, UiTransactionEncoding};
use tokio::time::sleep;

pub struct SolanaAggregator {
    client: RpcClient,
    transactions: Arc<Mutex<HashMap<Signature, TransactionDetails>>>,
}

impl SolanaAggregator {
    pub fn new(
        rpc_url: &str,
        transactions: Arc<Mutex<HashMap<Signature, TransactionDetails>>>,
    ) -> Self {
        // Create a new RPC client connected to the Solana RPC URL
        let client = RpcClient::new(rpc_url);

        SolanaAggregator {
            client,
            transactions,
        }
    }

    pub async fn fetch_transactions(&self) {
        // Set up the parameters for fetching signatures
        let mut last_signature: Option<Signature> = None;

        loop {
            // Fetch the latest signatures for finalized transactions
            let signature_results = self.client.get_signatures_for_address_with_config(
                &self
                    .client
                    .get_account_with_commitment(
                        &self.client.get_identity().unwrap(),
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
                    for signature_info in signatures.iter().rev() {
                        let signature = Signature::from_str(&signature_info.signature)
                            .expect("Invalid signature format");
                        match self
                            .client
                            .get_transaction(&signature, UiTransactionEncoding::Json)
                        {
                            Ok(transaction) => {
                                let block_time = transaction.block_time.unwrap_or(0);

                                if let EncodedTransaction::Json(transaction) =
                                    transaction.transaction.transaction
                                {
                                    if let UiMessage::Raw(message) = transaction.message {
                                        let sender = &message.account_keys[0];
                                        let receiver = &message.account_keys[1];
                                        let data = &message.instructions[0].data;

                                        let transaction_details = TransactionDetails {
                                            sender: sender.to_string(),
                                            receiver: receiver.to_string(),
                                            data: data.to_string(),
                                            timestamp: block_time,
                                        };
                                        tracing::info!(
                                            "📄 Storing transaction with signature {} ({})",
                                            signature,
                                            utils::format_time(block_time)
                                        );
                                        let mut transactions = self.transactions.lock().unwrap();
                                        transactions.insert(signature, transaction_details);
                                    }
                                }
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
    }
}
