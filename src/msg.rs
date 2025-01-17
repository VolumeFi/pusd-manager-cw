use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, CustomMsg};

#[allow(unused_imports)]
use crate::state::{BurnInfo, State};

#[cw_serde]
pub struct InstantiateMsg {
    pub retry_delay: u64,
    pub denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    // Register Jobs in hash map with chain_id as key and job_id as value
    RegisterJob {
        chain_id: String,
        job_id: String,
    },
    // Mint PUSD to recipient
    MintPusd {
        recipient: Addr,
        amount: u128,
    },
    // Receive PUSD and keep with nonce until withdrawn by owner
    Withdraw {
        chain_id: String,
        recipient: String,
    },
    // ReWithdraw PUSD by nonce
    ReWithdraw {
        nonce: u64,
    },
    // Burn PUSD by nonce
    BurnPusd {
        nonce: u64,
    },
    // Cancel Withdraw by nonce
    CancelWithdraw {
        nonce: u64,
    },
    // Update Config
    UpdateConfig {
        retry_delay: Option<u64>,
        owner: Option<Addr>,
    },
    // Set Paloma address of a chain
    SetPaloma {
        chain_id: String,
    },
    // Update Compass
    UpdateCompass {
        chain_id: String,
        new_compass: String,
    },
}

#[cw_serde]
pub enum PalomaMsg {
    /// Message struct for cross-chain calls.
    SchedulerMsg { execute_job: ExecuteJob },
    /// Message struct for tokenfactory calls.
    TokenFactoryMsg {
        create_denom: Option<CreateDenomMsg>,
        mint_tokens: Option<MintMsg>,
    },
}

#[cw_serde]
pub struct ExecuteJob {
    pub job_id: String,
    pub payload: Binary,
}

#[cw_serde]
pub struct CreateDenomMsg {
    pub subdenom: String,
    pub metadata: Metadata,
}

#[cw_serde]
pub struct DenomUnit {
    pub denom: String,
    pub exponent: u32,
    pub aliases: Vec<String>,
}

#[cw_serde]
pub struct Metadata {
    pub description: String,
    pub denom_units: Vec<DenomUnit>,
    pub base: String,
    pub display: String,
    pub name: String,
    pub symbol: String,
}

#[cw_serde]
pub struct MintMsg {
    pub denom: String,
    pub amount: u128,
    pub mint_to_address: String,
}

#[cw_serde]
pub struct BurnMsg {
    pub denom: String,
    pub amount: u128,
    /// burn_from_address must be set to "" for now.
    pub burn_from_address: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(State)]
    GetState {},

    #[returns(Vec<JobIdInfo>)]
    GetJobIds {},

    #[returns(String)]
    GetJobId { chain_id: String },

    #[returns(Vec<(u64, BurnInfo)>)]
    GetWithdrawList {},

    #[returns(BurnInfo)]
    GetBurnInfo { nonce: u64 },

    #[returns(bool)]
    ReWithdrawable {},
}

#[cw_serde]
pub struct JobIdInfo {
    pub chain_id: String,
    pub job_id: String,
}

impl CustomMsg for PalomaMsg {}
