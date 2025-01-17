# PUSD Manager CW

This repository contains a CosmWasm smart contract for managing PUSD tokens.

## Overview

The PUSD Manager CW smart contract provides functionalities to manage PUSD tokens, including minting, burning, and transferring tokens. It is built using the CosmWasm framework.

## Features

- Mint PUSD tokens
- Receive PUSD and keep until USDT release confirmed
- Burn PUSD tokens

## Requirements

- Rust
- Cargo
- CosmWasm
- wasm-pack

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/pusd-manager-cw.git
    cd pusd-manager-cw
    ```

2. Install dependencies:
    ```sh
    cargo build
    ```

## Usage

### Compile the contract

```sh
cargo wasm
```

### Run tests

```sh
cargo test
```

### Deploy the contract

Follow the instructions in the [CosmWasm documentation](https://docs.cosmwasm.com/docs/1.0/getting-started/compile-contract) to deploy the contract to a blockchain.

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

