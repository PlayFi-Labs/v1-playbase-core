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

### Coordination Module

#### Running the `coordination_module`

Navigate to the root directory of the project and run:

```bash
cargo run --package coordination_module
```

This will execute the coordination module functionality, which includes loading JSON objects, running the JSON comparator, and processing the resulting best JSON object with the fingerprint functionality.

#### Running the `json_comparator` setup

Navigate to the root directory of the project and run:

```bash
cargo run --bin json_comparator
```

This will execute the JSON comparator functionality within the coordination module.

#### Running the `fingerprint` setup

Navigate to the root directory of the project and run:


```bash
cargo run --bin fingerprint_runner
```

This will execute the fingerprint functionality within the coordination module, which interacts with the blockchain.

#### Running the `json` setup

Navigate to the root directory of the project and run:

```bash
cargo run --bin json_loader
```

This will execute the JSON loading functionality, which reads JSON objects from the specified directory and prints them out to verify they have been loaded correctly