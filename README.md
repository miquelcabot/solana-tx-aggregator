# Solana Blockchain Data Aggregator

## Overview

The **Solana Blockchain Data Aggregator** is a Rust-based application designed to retrieve, process, and expose Solana blockchain transaction data via a RESTful API. The application continuously monitors the Solana blockchain, starting from the current epoch, to aggregate real-time transaction data and make it accessible for further analysis and queries.

## Features

* **Data Retrieval**: Fetches transaction data from the Solana blockchain using Solana's RPC API.
* **Data Processing**: Efficiently parses transaction records to extract details such as sender, receiver, amount, and timestamp.
* **Real-Time Monitoring**: Continuously monitors the blockchain for new transactions, excluding historical data to focus on recent activity.
* **RESTful API**: Exposes the aggregated data via a RESTful API, supporting queries by transaction ID and date.
* **Concurrency**: Efficiently handles data processing and API requests concurrently using Rust's asynchronous programming capabilities.

## Architecture

The application is divided into several modules, each with a distinct responsibility:

* **Aggregator Module** (`aggregator.rs`): Manages the logic for aggregating and processing transactions, handling all interactions with the Solana RPC client, fetching signatures and transaction data.
* **API Module** (`api.rs`): Provides a RESTful API for accessing the aggregated transaction data.
* **Transaction Details Module** (`transaction_details.rs`): Defines the `TransactionDetails` struct to represent parsed transaction data.
* **Utils Module** (`utils.rs`): Contains helper functions for common tasks, such as date formatting.
* **Main** (`main.rs`): Initializes the application, configures the API server, and runs the data aggregator.

## Usage

### Prerequisites

* **Rust**: Ensure that you have Rust installed on your system. You can install Rust from [here](https://rustup.rs/).
* **Solana Node**: You can use a free Solana node provider like [Helius](https://www.helius.dev/) or the [official Solana  RPCs](https://solana.com/rpc).

### Command-Line Interface

```bash
Usage: solforge-case-study [OPTIONS]

Options:
  -r, --rpc-url <RPC_URL>              The address of the RPC URL for Solana [default: https://api.devnet.solana.com/]
  -l, --local-address <LOCAL_ADDRESS>  The address of this local RESTful API server [default: 0.0.0.0:8080]
  -h, --help                           Print help
  -V, --version                        Print version
```

### Running the Application

1. Clone the repository and navigate to the project directory.

```bash
git clone https://github.com/miquelcabot/solforge-case-study.git
cd solforge-case-study
```

2. Build and run the application.

```bash
cargo run -- --rpc-url <RPC_URL> --local-address <LOCAL_ADDRESS>
```

Replace `<RPC_URL>` with the Solana RPC URL you wish to use and `<LOCAL_ADDRESS>` with the desired address for the local RESTful API server.

3. The application will start fetching transactions from the Solana blockchain and serving the data via the RESTful API.

### API Endpoints

The following API endpoints are available:

* **Get Transaction by ID**:

```bash
GET /transactions?id=<transaction_id>
```

Example:

```bash
curl http://localhost:8080/transactions?id=4CqYTMNtGpWjk67Ntq9QtDHZNaDeqYwhbh6cMVx7Qx6Y4b43kgsHP8t4TJbdrWf5kD4xuWNXhFLZfo4H6GBmxXzG

```

* **Get Transactions by Date**:

```bash
GET /transactions?day=<dd/mm/yyyy>
```

Example:

```bash
curl http://localhost:8080/transactions?day=23/05/2023
```

## Design Decisions

* **Modularity**: The application is designed with a modular structure to ensure separation of concerns and ease of maintenance.
* **Concurrency**: The application uses Rust's `tokio` library to handle data fetching and API requests concurrently, ensuring responsiveness even under high load.
* **Real-Time Monitoring**: The application focuses on real-time data aggregation by continuously monitoring the current epoch, excluding historical data to optimize performance.

## Testing

The application includes unit tests to ensure the reliability of the core components. To run the tests:

```bash
cargo test
```

## Contact

For any questions or inquiries, please contact [miquel@cabot.dev](mailto:miquel@cabot.dev).
