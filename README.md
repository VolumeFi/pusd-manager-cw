# PUSD Manager CW

This repository contains a CosmWasm smart contract for managing PUSD tokens.

## Overview

The PUSD Manager CW smart contract provides functionalities to manage PUSD tokens, including minting, burning, and transferring tokens. It is built using the CosmWasm framework.

## Features

- Register Chain
  Register chain settings in hash map with chain_id as key
 
- Mint Pusd
  Mint PUSD denom when user deposits USDT on EVM

- Withdraw
  Put PUSD and send release USDT message on EVM chain

- ReWithdraw
  Resend withdraw message in case the previous message failed

- BurnPusd
  Burn PUSD denom when the release USDT message succeed

- CancelWithdraw
  Cancel withdraw PUSD in case the previous message failed

- UpdateConfig
  Update configs including retry_delay and owner

- SetPaloma
  Set paloma address on EVM Vyper contract

- UpdateCompass
  Update compass address on EVM Vyper contract

- UpdateRefundWallet
  Update refund wallet address on EVM Vyper contract

## Requirements

- Rust
- Cargo
- CosmWasm

## Usage

### Compile the contract

```sh
cargo wasm
```

### Run tests

```sh
cargo test
```

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

