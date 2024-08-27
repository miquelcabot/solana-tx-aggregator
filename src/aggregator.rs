use crate::transaction_details::TransactionDetails;
use crate::utils;

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use solana_client::client_error::ClientError;
use solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient};
use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use solana_transaction_status::{EncodedTransaction, UiMessage, UiTransactionEncoding};
use tokio::time::sleep;

const SLEEP_DURATION: u64 = 1;

/// Solana Aggregator
///
/// This struct represents a Solana aggregator.
///
/// # Fields
/// * `client` - The RPC client
/// * `transactions` - The transactions, stored in a mutexed hash map
pub struct SolanaAggregator {
    client: RpcClient,
    transactions: Arc<Mutex<HashMap<Signature, TransactionDetails>>>,
}

impl SolanaAggregator {
    /// Create a new Solana aggregator
    ///
    /// This function creates a new Solana aggregator.
    ///
    /// # Arguments
    /// * `rpc_url` - The RPC URL for Solana
    /// * `transactions` - The transactions mutexed hash map
    ///
    /// # Returns
    /// A new Solana aggregator
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

    /// Fetch transactions
    ///
    /// This function fetches transactions from the Solana network.
    ///
    /// # Arguments
    /// * `self` - The Solana aggregator
    pub async fn fetch_transactions(&self) {
        // Set up the parameters for fetching signatures
        let mut last_signature: Option<Signature> = None;

        loop {
            // Fetch the latest signatures for finalized transactions
            match self.fetch_signatures(last_signature).await {
                Ok(signatures) => {
                    self.process_signatures(signatures).await;
                    last_signature = self.update_last_signature();
                }
                Err(err) => {
                    tracing::error!("❌ Error fetching signatures: {}", err);
                }
            }

            // Wait for a bit before fetching more signatures
            sleep(Duration::from_secs(SLEEP_DURATION)).await;
        }
    }

    /// Fetch signatures
    ///
    /// This function fetches signatures from the Solana network.
    ///
    /// # Arguments
    /// * `self` - The Solana aggregator
    /// * `last_signature` - The last signature
    ///
    /// # Returns
    /// A vector of signatures
    async fn fetch_signatures(
        &self,
        last_signature: Option<Signature>,
    ) -> Result<Vec<RpcConfirmedTransactionStatusWithSignature>, ClientError> {
        let account = self
            .client
            .get_account_with_commitment(
                &self.client.get_identity().unwrap(),
                CommitmentConfig::finalized(),
            )
            .unwrap()
            .value
            .unwrap()
            .owner;
        let config = GetConfirmedSignaturesForAddress2Config {
            before: None,
            until: last_signature,
            limit: Some(1000),
            commitment: Some(CommitmentConfig::finalized()),
        };

        self.client
            .get_signatures_for_address_with_config(&account, config)
    }

    /// Process signatures
    ///  
    /// This function processes signatures from the Solana network.
    ///
    /// # Arguments
    /// * `self` - The Solana aggregator
    /// * `signatures` - The signatures to process
    async fn process_signatures(
        &self,
        signatures: Vec<solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature>,
    ) {
        for signature_info in signatures.iter().rev() {
            let signature =
                Signature::from_str(&signature_info.signature).expect("Invalid signature format");
            match self.fetch_and_process_transaction(&signature).await {
                Ok(_) => {}
                Err(err) => {
                    tracing::error!("❌ Error processing transaction: {}", err);
                }
            }
        }
    }

    /// Fetch and process a transaction
    ///
    /// This function fetches and processes a transaction from the Solana network.
    ///
    /// # Arguments
    /// * `self` - The Solana aggregator
    /// * `signature` - The signature of the transaction
    async fn fetch_and_process_transaction(
        &self,
        signature: &Signature,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let transaction = self
            .client
            .get_transaction(signature, UiTransactionEncoding::Json)?;

        let block_time = transaction.block_time.unwrap_or(0);

        if let EncodedTransaction::Json(transaction) = transaction.transaction.transaction {
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
                transactions.insert(*signature, transaction_details);
            }
        }

        Ok(())
    }

    /// Update the last signature
    ///
    /// This function updates the last signature.
    ///
    /// # Arguments
    /// * `self` - The Solana aggregator
    fn update_last_signature(&self) -> Option<Signature> {
        let transactions = self.transactions.lock().unwrap();
        transactions.keys().last().cloned()
    }
}
