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

Create a `.env` file in the root of the project by copying the provided `.env.example` file. You can use the following command:

```sh
cp .env.example .env
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

### Running Tests

To run the tests for the project, navigate to the root directory and execute:

```bash
cargo test
```

This will run all the tests defined in the project, ensuring that everything is working as expected