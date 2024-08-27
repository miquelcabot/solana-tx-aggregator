use crate::transaction_details::TransactionDetails;

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use solana_sdk::signature::Signature;
use warp::Filter;

/// Create the RESTful API
///
/// This function creates the RESTful API.
///
/// # Arguments
/// * `transactions` - The transactions hash map
///
/// # Returns
/// A warp filter
pub fn create_api(
    transactions: Arc<Mutex<HashMap<Signature, TransactionDetails>>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("transactions")
        .and(warp::query::<HashMap<String, String>>())
        .and(with_transactions(transactions))
        .map(
            |params: HashMap<String, String>,
             transactions: Arc<Mutex<HashMap<Signature, TransactionDetails>>>| {
                let transactions = transactions.lock().unwrap();

                if let Some(signature) = params.get("id") {
                    let signature =
                        Signature::from_str(signature).expect("Invalid signature format");
                    if let Some(transaction) = transactions.get(&signature) {
                        return warp::reply::json(transaction);
                    } else {
                        return warp::reply::json(&None::<TransactionDetails>); // Return null if not found
                    }
                }

                if let Some(day) = params.get("day") {
                    let date = chrono::NaiveDate::parse_from_str(day, "%d/%m/%Y")
                        .expect("Invalid date format");
                    let transactions_for_day: Vec<&TransactionDetails> = transactions
                        .values()
                        .filter(|transaction| {
                            chrono::DateTime::from_timestamp(transaction.timestamp, 0)
                                .unwrap()
                                .date_naive()
                                == date
                        })
                        .collect();
                    return warp::reply::json(&transactions_for_day);
                }

                warp::reply::json(&"Invalid query parameters")
            },
        )
}

/// Return a mutable reference to the transactions hash map
///
/// This function returns a mutable reference to the transactions hash map.
///
/// # Arguments
/// * `transactions` - The transactions hash map
///
/// # Returns
/// A warp filter
fn with_transactions(
    transactions: Arc<Mutex<HashMap<Signature, TransactionDetails>>>,
) -> impl Filter<
    Extract = (Arc<Mutex<HashMap<Signature, TransactionDetails>>>,),
    Error = std::convert::Infallible,
> + Clone {
    warp::any().map(move || transactions.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::signature::Signature;
    use std::sync::{Arc, Mutex};
    use warp::test::request;

    #[tokio::test]
    async fn test_get_transaction_by_id() {
        let mut transactions = HashMap::new();
        let signature = Signature::new_unique();
        transactions.insert(
            signature,
            TransactionDetails {
                sender: "SenderPubkey".to_string(),
                receiver: "ReceiverPubkey".to_string(),
                data: "some_data".to_string(),
                timestamp: 1620000000,
            },
        );

        let transactions = Arc::new(Mutex::new(transactions));
        let api = create_api(transactions);

        let response = request()
            .path(&format!("/transactions?id={}", signature))
            .reply(&api)
            .await;

        assert_eq!(response.status(), 200);
        let body_str = std::str::from_utf8(response.body()).unwrap(); // Convert body to &str
        assert!(body_str.contains("SenderPubkey"));
    }

    #[tokio::test]
    async fn test_get_transaction_by_id_not_found() {
        let signature = Signature::new_unique();
        let transactions = Arc::new(Mutex::new(HashMap::new()));
        let api = create_api(transactions);

        let response = request()
            .path(&format!("/transactions?id={}", signature))
            .reply(&api)
            .await;

        assert_eq!(response.status(), 200);
        let body_str = std::str::from_utf8(response.body()).unwrap(); // Convert body to &str
        assert_eq!(body_str, "null");
    }

    #[tokio::test]
    async fn test_invalid_query_parameters() {
        let transactions = Arc::new(Mutex::new(HashMap::new()));
        let api = create_api(transactions);

        let response = request().path("/transactions").reply(&api).await;

        assert_eq!(response.status(), 200);
        let body_str = std::str::from_utf8(response.body()).unwrap(); // Convert body to &str
        println!("{}", body_str);
        assert_eq!(body_str, "\"Invalid query parameters\"");
    }
}
