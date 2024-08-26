use std::collections::HashMap;

use solana_client::rpc_client::RpcClient;
use solana_sdk::clock::Epoch;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;

pub struct SolanaAggregator {
    client: RpcClient,
    current_epoch: Epoch,
    transactions: HashMap<Signature, Transaction>,
}

impl SolanaAggregator {
    pub fn new(rpc_url: &str) -> Self {
        let client = RpcClient::new(rpc_url);
        let current_epoch = client.get_epoch_info().unwrap().epoch;
        let transactions = HashMap::new();

        Self {
            client,
            current_epoch,
            transactions,
        }
    }
}
