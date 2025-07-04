use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, CustomMsg, Uint128};

#[allow(unused_imports)]
use crate::state::{BurnInfo, ChainSetting, State};

#[cw_serde]
pub struct MigrateMsg {
    pub minter: Addr,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub retry_delay: u64,
    pub minter: Addr,
    pub denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    // Register Jobs in hash map with chain_id as key and job_id as value
    RegisterChain {
        chain_id: String,
        chain_setting: ChainSetting,
    },
    SetBridge {
        chain_reference_id: String,
        erc20_address: String,
    },
    // Mint PUSD to recipient
    MintPusd {
        recipient: Addr,
        amount: Uint128,
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

    UnmintPusd {
        amount: Uint128,
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
    // Update Refund Wallet
    UpdateRefundWallet {
        chain_id: String,
        new_refund_wallet: String,
    },
    UpdateRedemptionFee {
        chain_id: String,
        new_redemption_fee: Uint128,
    },
}

#[cw_serde]
pub enum PalomaMsg {
    /// Message struct for cross-chain calls.
    SchedulerMsg {
        execute_job: ExecuteJob,
    },
    SkywayMsg {
        set_erc20_to_denom: SetErc20ToDenom,
    },
    TokenFactoryMsg {
        change_admin: ChangeAdminMsg,
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
    pub amount: Uint128,
    pub mint_to_address: String,
}

#[cw_serde]
pub struct BurnMsg {
    pub denom: String,
    pub amount: Uint128,
    /// burn_from_address must be set to "" for now.
    pub burn_from_address: String,
}

#[cw_serde]
pub struct ChangeAdminMsg {
    pub denom: String,
    pub new_admin_address: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(State)]
    GetState {},

    #[returns(Vec<ChainSettingInfo>)]
    GetChainSettings {},

    #[returns(String)]
    GetJobId { chain_id: String },

    #[returns(Vec<(u64, BurnInfo)>)]
    GetWithdrawList {},

    #[returns(BurnInfo)]
    GetBurnInfo { nonce: u64 },

    #[returns(bool)]
    ReWithdrawable {},

    #[returns(BalanceResponse)]
    PusdBalance {},
}

#[cw_serde]
pub struct ChainSettingInfo {
    pub chain_id: String,
    pub job_id: String,
    pub minimum_amount: Uint128,
}

#[cw_serde]
pub struct SetErc20ToDenom {
    pub erc20_address: String,
    pub token_denom: String,
    pub chain_reference_id: String,
}

#[cw_serde]
pub struct BalanceResponse {
    pub balance: Uint128,
}

impl CustomMsg for PalomaMsg {}
