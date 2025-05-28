use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub retry_delay: u64,
    pub owner: Addr,
    pub minter: Addr,
    pub denom: String,
    pub last_nonce: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct BurnInfo {
    pub chain_id: String,
    pub burner: Addr,
    pub recipient: String,
    pub amount: u128,
    pub timestamp: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ChainSetting {
    pub job_id: String,
    pub minimum_amount: Uint128,
}

pub const TX_TIMESTAMP: Map<(u64, String), Timestamp> = Map::new("tx_timestamp");
pub const CHAIN_SETTINGS: Map<String, ChainSetting> = Map::new("chain_settings");
pub const STATE: Item<State> = Item::new("state");
pub const WITHDRAW_LIST: Map<u64, BurnInfo> = Map::new("burn_list");
