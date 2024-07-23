# PlayBase Core V1

## Overview

PlayBase is a decentralized AI data processing and storage network that validates, processes, and stores game data for any, and all games. It enables a wide range of Web3 experiences — like decentralized tournaments, in-game prediction markets, and player-owned economies — on the most popular games.

## Project Structure

The project is organized into several modules, each with its own responsibilities:

- `ai_module`: Handles AI-related functionalities.
- `coordination_module`: Manages coordination tasks.
- `zk_module`: Handles zero-knowledge proofs and related cryptographic tasks.
- `storage_module`: Manages data storage.

## Setup

### Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Environment Variables

Create a `.env` file in the root of the project with the following content:

```dotenv
FINGERPRINT_PROXY_SC=your_contract_address
ZKSYNC_SEPOLIA_PRIVATE_KEY=your_private_key
ZKSYNC_URL=https://sepolia.era.zksync.dev
```

### Build the Project

Navigate to the root directory of the project and run:

```bash
cargo build
```

### Usage

#### Running the `json_comparator`

Navigate to the root directory of the project and run:

```bash
cargo run --package json_comparator
```

This will execute the JSON comparator functionality within the coordination module.

#### Running the `fingerprint`

Navigate to the root directory of the project and run:


```bash
cargo run --package fingerprint
```

This will execute the fingerprint functionality within the coordination module, which interacts with the blockchain.



