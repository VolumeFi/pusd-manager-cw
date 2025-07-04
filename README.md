# PUSD Manager CW

This repository contains a CosmWasm smart contract for managing PUSD tokens with cross-chain functionality.

## Overview

The PUSD Manager CW smart contract provides functionalities to manage PUSD tokens, including minting, burning, and transferring tokens across different blockchain networks. It is built using the CosmWasm framework and integrates with the Paloma network for cross-chain operations.

## Architecture

The contract maintains the following key data structures:
- **State**: Global contract configuration including owner, minter, retry delay, and token denomination
- **ChainSettings**: Per-chain configuration including job IDs and minimum withdrawal amounts
- **WithdrawList**: Pending withdrawal requests with nonces for tracking
- **BurnInfo**: Detailed information about each withdrawal request

## Security Considerations

### Access Control
- **Owner**: Has administrative privileges including configuration updates and chain registration
- **Minter**: Can mint and unmint PUSD tokens
- **Users**: Can initiate withdrawals and cancel their own withdrawal requests

### Critical Security Features
- Nonce-based withdrawal tracking prevents replay attacks
- Retry delay mechanism prevents rapid withdrawal attempts
- Minimum amount validation prevents dust attacks
- Authorization checks on all administrative functions

## Function Documentation

### Contract Lifecycle Functions

#### `instantiate`
**Purpose**: Initializes the contract with initial configuration
**Access**: Public (contract deployment)
**Parameters**:
- `retry_delay`: Time delay before withdrawal can be retried (seconds)
- `minter`: Address authorized to mint/unmint PUSD tokens
- `denom`: Token denomination string

**Security**: Requires funds to be sent during instantiation
**Example**:
```json
{
  "retry_delay": 3600,
  "minter": "cosmos1...",
  "denom": "factory/cosmos1.../pusd"
}
```

#### `migrate`
**Purpose**: Handles contract upgrades and state migration
**Access**: Owner only
**Parameters**:
- `minter`: New minter address for the upgraded contract

**Security**: Migrates existing state while updating minter permissions
**Example**:
```json
{
  "minter": "cosmos1..."
}
```

### Administrative Functions

#### `register_chain`
**Purpose**: Registers a new blockchain network for cross-chain operations
**Access**: Owner only
**Parameters**:
- `chain_id`: Unique identifier for the blockchain network
- `chain_setting`: Configuration including job_id and minimum_amount

**Security**: Only owner can register new chains
**Example**:
```json
{
  "chain_id": "ethereum",
  "chain_setting": {
    "job_id": "withdraw_job_001",
    "minimum_amount": "1000000"
  }
}
```

#### `set_bridge`
**Purpose**: Configures ERC20 token mapping for cross-chain bridge
**Access**: Owner only
**Parameters**:
- `chain_reference_id`: Target chain identifier
- `erc20_address`: ERC20 contract address on target chain

**Security**: Only owner can configure bridge settings
**Example**:
```json
{
  "chain_reference_id": "ethereum",
  "erc20_address": "0x1234567890123456789012345678901234567890"
}
```

#### `update_config`
**Purpose**: Updates global contract configuration
**Access**: Owner only
**Parameters**:
- `retry_delay`: Optional new retry delay (must be > 0)
- `owner`: Optional new owner address

**Security**: Only current owner can update configuration
**Example**:
```json
{
  "retry_delay": 7200,
  "owner": "cosmos1..."
}
```

### Token Management Functions

#### `mint_pusd`
**Purpose**: Mints PUSD tokens to a specified recipient
**Access**: Owner only
**Parameters**:
- `recipient`: Address to receive minted tokens
- `amount`: Amount of PUSD tokens to mint

**Security**: Only owner can mint tokens; amount must be > 0
**Example**:
```json
{
  "recipient": "cosmos1...",
  "amount": "1000000000"
}
```

#### `unmint_pusd`
**Purpose**: Burns PUSD tokens from the minter's balance
**Access**: Minter only
**Parameters**:
- `amount`: Amount of PUSD tokens to burn

**Security**: Only minter can unmint tokens; amount must be > 0
**Example**:
```json
{
  "amount": "1000000000"
}
```

### Cross-Chain Withdrawal Functions

#### `withdraw`
**Purpose**: Initiates a cross-chain withdrawal of PUSD tokens
**Access**: Any user with PUSD tokens
**Parameters**:
- `chain_id`: Target blockchain network
- `recipient`: Recipient address on target chain

**Security**: 
- User must send PUSD tokens with the transaction
- Amount must exceed chain's minimum withdrawal amount
- Creates unique nonce for tracking
**Example**:
```json
{
  "chain_id": "ethereum",
  "recipient": "0x1234567890123456789012345678901234567890"
}
```

#### `re_withdraw`
**Purpose**: Retries a failed withdrawal request
**Access**: Original withdrawal initiator
**Parameters**:
- `nonce`: Unique identifier of the withdrawal request

**Security**: 
- Only original initiator can retry
- Must wait for retry_delay period
- Updates timestamp to prevent rapid retries
**Example**:
```json
{
  "nonce": 123
}
```

#### `burn_pusd`
**Purpose**: Burns PUSD tokens after successful cross-chain withdrawal
**Access**: Owner only
**Parameters**:
- `nonce`: Unique identifier of the withdrawal request

**Security**: Only owner can burn tokens; removes from withdraw list
**Example**:
```json
{
  "nonce": 123
}
```

#### `cancel_withdraw`
**Purpose**: Cancels a pending withdrawal and returns tokens
**Access**: Original withdrawal initiator
**Parameters**:
- `nonce`: Unique identifier of the withdrawal request

**Security**: 
- Only original initiator can cancel
- Must wait for retry_delay period
- Returns tokens to initiator
**Example**:
```json
{
  "nonce": 123
}
```

### EVM Contract Management Functions

#### `set_paloma`
**Purpose**: Sets Paloma address on EVM Vyper contract
**Access**: Owner only
**Parameters**:
- `chain_id`: Target blockchain network

**Security**: Only owner can update Paloma address
**Example**:
```json
{
  "chain_id": "ethereum"
}
```

#### `update_compass`
**Purpose**: Updates compass address on EVM Vyper contract
**Access**: Owner only
**Parameters**:
- `chain_id`: Target blockchain network
- `new_compass`: New compass contract address

**Security**: Only owner can update compass address
**Example**:
```json
{
  "chain_id": "ethereum",
  "new_compass": "0x1234567890123456789012345678901234567890"
}
```

#### `update_refund_wallet`
**Purpose**: Updates refund wallet address on EVM Vyper contract
**Access**: Owner only
**Parameters**:
- `chain_id`: Target blockchain network
- `new_refund_wallet`: New refund wallet address

**Security**: Only owner can update refund wallet
**Example**:
```json
{
  "chain_id": "ethereum",
  "new_refund_wallet": "0x1234567890123456789012345678901234567890"
}
```

#### `update_redemption_fee`
**Purpose**: Updates redemption fee on EVM Vyper contract
**Access**: Owner only
**Parameters**:
- `chain_id`: Target blockchain network
- `new_redemption_fee`: New redemption fee amount

**Security**: Only owner can update redemption fee
**Example**:
```json
{
  "chain_id": "ethereum",
  "new_redemption_fee": "1000000"
}
```

### Query Functions

#### `get_state`
**Purpose**: Returns current contract state
**Access**: Public
**Returns**: State object with owner, minter, retry_delay, denom, and last_nonce

#### `get_chain_settings`
**Purpose**: Returns all registered chain configurations
**Access**: Public
**Returns**: Array of ChainSettingInfo objects

#### `get_job_id`
**Purpose**: Returns job ID for a specific chain
**Access**: Public
**Parameters**:
- `chain_id`: Target blockchain network
**Returns**: Job ID string

#### `get_withdraw_list`
**Purpose**: Returns all pending withdrawal requests
**Access**: Public
**Returns**: Array of (nonce, BurnInfo) tuples

#### `get_burn_info`
**Purpose**: Returns details of a specific withdrawal request
**Access**: Public
**Parameters**:
- `nonce`: Unique identifier of the withdrawal request
**Returns**: BurnInfo object

#### `re_withdrawable`
**Purpose**: Checks if any withdrawals are eligible for retry
**Access**: Public
**Returns**: Boolean indicating if withdrawals can be retried

#### `pusd_balance`
**Purpose**: Returns contract's PUSD token balance
**Access**: Public
**Returns**: BalanceResponse with current balance

## State Variables

### Global State
```rust
pub struct State {
    pub retry_delay: u64,        // Time delay before withdrawal retry
    pub owner: Addr,             // Contract owner address
    pub minter: Addr,            // Token minter address
    pub denom: String,           // PUSD token denomination
    pub last_nonce: u64,         // Last used nonce for withdrawals
}
```

### Withdrawal Information
```rust
pub struct BurnInfo {
    pub chain_id: String,        // Target blockchain network
    pub burner: Addr,            // User who initiated withdrawal
    pub recipient: String,       // Recipient address on target chain
    pub amount: u128,            // Withdrawal amount
    pub timestamp: Timestamp,    // Withdrawal timestamp
}
```

### Chain Configuration
```rust
pub struct ChainSetting {
    pub job_id: String,          // Paloma job identifier
    pub minimum_amount: Uint128, // Minimum withdrawal amount
}
```

## Storage Layout

- `STATE`: Global contract state
- `CHAIN_SETTINGS`: Chain-specific configurations
- `WITHDRAW_LIST`: Pending withdrawal requests indexed by nonce
- `TX_TIMESTAMP`: Transaction timestamps (unused in current implementation)

## Error Handling

The contract uses custom error types defined in `error.rs`:
- `MigrationFailed`: Contract migration errors
- `Unauthorized`: Access control violations
- `InvalidAmount`: Amount validation failures
- `InvalidChainId`: Chain ID validation errors

## Requirements

- Rust 1.70+
- Cargo
- CosmWasm 1.4+
- Paloma network integration

## Usage

### Compile the contract

```sh
cargo wasm
```

### Run tests

```sh
cargo test
```

### Deploy

```sh
# Deploy with initial configuration
wasmd tx wasm instantiate <code_id> '{"retry_delay": 3600, "minter": "cosmos1...", "denom": "factory/cosmos1.../pusd"}' --from <key> --label "PUSD Manager" --gas auto --gas-adjustment 1.3
```

## Security Audit Checklist

When auditing this contract, pay special attention to:

1. **Access Control**: Verify all administrative functions are properly restricted
2. **Nonce Management**: Ensure nonce uniqueness and proper incrementing
3. **Cross-Chain Validation**: Verify chain_id and recipient address validation
4. **Amount Validation**: Check minimum amount enforcement and overflow protection
5. **Timing Attacks**: Verify retry delay mechanism effectiveness
6. **State Consistency**: Ensure proper state updates across all operations
7. **Error Handling**: Verify graceful error handling and proper rollbacks
8. **Reentrancy**: Check for potential reentrancy vulnerabilities
9. **Gas Optimization**: Verify gas usage patterns and limits
10. **Integration Security**: Review Paloma network integration security

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

