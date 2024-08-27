use chrono::prelude::*;

/// Format the transaction time for logging
///
/// This function takes a timestamp and returns a formatted string.
///
/// # Arguments
/// * `timestamp` - The timestamp to format
///
/// # Returns
/// A formatted string
pub fn format_time(timestamp: i64) -> String {
    // Format the transaction time for logging
    let datetime = DateTime::from_timestamp(timestamp, 0);
    let newdate = datetime.unwrap().format("%Y-%m-%d %H:%M:%S");

    newdate.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        let timestamp = 1620000000; // This corresponds to 2021-05-03 00:00:00 UTC
        let formatted_time = format_time(timestamp);
        assert_eq!(formatted_time, "2021-05-03 00:00:00");
    }
}
