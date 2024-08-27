#[derive(Debug, Clone)]
pub struct TransactionDetails {
    pub sender: String,
    pub receiver: String,
    pub data: String,
    pub timestamp: i64,
}
