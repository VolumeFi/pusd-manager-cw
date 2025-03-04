use std::collections::BTreeMap;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, CanonicalAddr, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Order, Response, StdResult, Uint128,
};
use ethabi::{Address, Contract, Function, Param, ParamType, StateMutability, Token, Uint};

use crate::error::ContractError;
use crate::msg::{
    ChainSettingInfo, CreateDenomMsg, DenomUnit, ExecuteJob, ExecuteMsg, InstantiateMsg, Metadata,
    MintMsg, PalomaMsg, QueryMsg, SetErc20ToDenom,
};
use crate::state::{BurnInfo, State, CHAIN_SETTINGS, STATE, WITHDRAW_LIST};
use std::str::FromStr;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<PalomaMsg>, ContractError> {
    assert!(!info.funds.is_empty(), "Insufficient funds");
    let subdenom = "upusd";
    let creator = env.contract.address.to_string();
    let denom = "factory/".to_string() + creator.as_str() + "/" + subdenom;
    let state = State {
        retry_delay: msg.retry_delay,
        owner: info.sender.clone(),
        denom: denom.clone(),
        last_nonce: 0,
    };
    STATE.save(deps.storage, &state)?;
    let metadata: Metadata = Metadata {
        description: "Paloma USD stablecoin".to_string(),
        denom_units: vec![
            DenomUnit {
                denom: denom.clone(),
                exponent: 0,
                aliases: vec![],
            },
            DenomUnit {
                denom: "pusd".to_string(),
                exponent: 6,
                aliases: vec![],
            },
        ],
        name: "Paloma USD".to_string(),
        symbol: "PUSD".to_string(),
        base: denom.clone(),
        display: "pusd".to_string(),
    };
    Ok(Response::new()
        .add_message(CosmosMsg::Custom(PalomaMsg::TokenFactoryMsg {
            create_denom: Some(CreateDenomMsg {
                subdenom: subdenom.to_string(),
                metadata,
            }),
            mint_tokens: None,
        }))
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("action", "create_pusd")
        .add_attribute("denom", denom))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<PalomaMsg>, ContractError> {
    match msg {
        ExecuteMsg::RegisterChain {
            chain_id,
            chain_setting,
        } => {
            // ACTION: Implement RegisterJob
            assert!(
                info.sender == STATE.load(deps.storage)?.owner,
                "Unauthorized"
            );
            assert!(!chain_id.is_empty(), "Chain ID cannot be empty");
            assert!(!chain_setting.job_id.is_empty(), "Job ID cannot be empty");
            CHAIN_SETTINGS.save(deps.storage, chain_id.clone(), &chain_setting)?;
            Ok(Response::new().add_attributes(vec![
                ("action", "register_job"),
                ("chain_id", &chain_id),
                ("job_id", &chain_setting.job_id),
                ("minimum_amount", &chain_setting.minimum_amount.to_string()),
            ]))
        }
        ExecuteMsg::SetBrigde {
            chain_reference_id,
            erc20_address,
        } => {
            assert!(
                info.sender == STATE.load(deps.storage)?.owner,
                "Unauthorized"
            );
            let token_denom = STATE.load(deps.storage)?.denom.clone();
            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SkywayMsg {
                    set_erc20_to_denom: SetErc20ToDenom {
                        erc20_address,
                        token_denom,
                        chain_reference_id,
                    },
                }))
                .add_attribute("action", "set_bridge"))
        }
        ExecuteMsg::MintPusd { recipient, amount } => {
            // ACTION: Implement MintPusd
            assert!(
                info.sender == STATE.load(deps.storage)?.owner,
                "Unauthorized"
            );

            assert!(!amount.is_zero(), "Amount must be greater than 0");

            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::TokenFactoryMsg {
                    create_denom: None,
                    mint_tokens: Some(MintMsg {
                        denom: STATE.load(deps.storage)?.denom,
                        amount,
                        mint_to_address: recipient.to_string(),
                    }),
                }))
                .add_attributes(vec![
                    ("action", "mint_pusd"),
                    ("recipient", recipient.as_str()),
                    ("amount", &amount.to_string()),
                ]))
        }
        ExecuteMsg::Withdraw {
            chain_id,
            recipient,
        } => {
            let nonce = STATE.load(deps.storage)?.last_nonce + 1;

            let mut amount: Uint128 = Uint128::zero();
            info.funds.iter().for_each(|coin: &Coin| {
                if coin.denom == STATE.load(deps.storage).unwrap().denom {
                    amount = coin.amount;
                }
            });
            let chain_setting = CHAIN_SETTINGS.load(deps.storage, chain_id.clone())?;
            assert!(
                amount > chain_setting.minimum_amount,
                "Amount must be greater than minimum amount"
            );
            let burn_info = BurnInfo {
                chain_id: chain_id.clone(),
                burner: info.sender.clone(),
                recipient: recipient.clone(),
                amount: info.funds[0].amount.u128(),
                timestamp: env.block.time,
            };

            WITHDRAW_LIST.save(deps.storage, nonce, &burn_info)?;

            STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
                state.last_nonce = nonce;
                Ok(state)
            })?;

            #[allow(deprecated)]
            let contract: Contract = Contract {
                constructor: None,
                functions: BTreeMap::from_iter(vec![(
                    "withdraw".to_string(),
                    vec![Function {
                        name: "withdraw".to_string(),
                        inputs: vec![
                            Param {
                                name: "sender".to_string(),
                                kind: ParamType::FixedBytes(32),
                                internal_type: None,
                            },
                            Param {
                                name: "recipient".to_string(),
                                kind: ParamType::Address,
                                internal_type: None,
                            },
                            Param {
                                name: "amount".to_string(),
                                kind: ParamType::Uint(256),
                                internal_type: None,
                            },
                            Param {
                                name: "nonce".to_string(),
                                kind: ParamType::Uint(256),
                                internal_type: None,
                            },
                        ],
                        outputs: Vec::new(),
                        constant: None,
                        state_mutability: StateMutability::NonPayable,
                    }],
                )]),
                events: BTreeMap::new(),
                errors: BTreeMap::new(),
                receive: false,
                fallback: false,
            };
            let canonical_addr: CanonicalAddr = deps.api.addr_canonicalize(info.sender.as_str())?;
            let tokens = &[
                Token::FixedBytes(canonical_addr.as_slice().to_vec()),
                Token::Address(Address::from_str(recipient.as_str()).unwrap()),
                Token::Uint(Uint::from_big_endian(&amount.to_be_bytes())),
                Token::Uint(Uint::from_big_endian(&nonce.to_be_bytes())),
            ];

            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                    execute_job: ExecuteJob {
                        job_id: CHAIN_SETTINGS.load(deps.storage, chain_id.clone())?.job_id,
                        payload: Binary::new(
                            contract
                                .function("withdraw")
                                .unwrap()
                                .encode_input(tokens.as_slice())
                                .unwrap(),
                        ),
                    },
                }))
                .add_attributes(vec![
                    ("action", "withdraw"),
                    ("chain_id", &chain_id),
                    ("recipient", recipient.as_str()),
                    ("nonce", &nonce.to_string()),
                ]))
        }
        ExecuteMsg::BurnPusd { nonce } => {
            // ACTION: Implement BurnPusd
            let burn_info = WITHDRAW_LIST.load(deps.storage, nonce)?;
            assert!(
                STATE.load(deps.storage)?.owner == info.sender,
                "Unauthorized"
            );

            WITHDRAW_LIST.remove(deps.storage, nonce);

            Ok(Response::new()
                .add_message(CosmosMsg::Bank(BankMsg::Burn {
                    amount: vec![Coin {
                        denom: STATE.load(deps.storage)?.denom,
                        amount: Uint128::from(burn_info.amount),
                    }],
                }))
                .add_attributes(vec![("action", "burn_pusd"), ("nonce", &nonce.to_string())]))
        }
        ExecuteMsg::ReWithdraw { nonce } => {
            // ACTION: Implement ReWithdraw
            let burn_info = WITHDRAW_LIST.load(deps.storage, nonce)?;
            assert!(
                burn_info
                    .timestamp
                    .plus_seconds(STATE.load(deps.storage)?.retry_delay)
                    < env.block.time,
                "Retry delay not reached"
            );

            #[allow(deprecated)]
            let contract: Contract = Contract {
                constructor: None,
                functions: BTreeMap::from_iter(vec![(
                    "withdraw".to_string(),
                    vec![Function {
                        name: "withdraw".to_string(),
                        inputs: vec![
                            Param {
                                name: "sender".to_string(),
                                kind: ParamType::FixedBytes(32),
                                internal_type: None,
                            },
                            Param {
                                name: "recipient".to_string(),
                                kind: ParamType::Address,
                                internal_type: None,
                            },
                            Param {
                                name: "amount".to_string(),
                                kind: ParamType::Uint(256),
                                internal_type: None,
                            },
                            Param {
                                name: "nonce".to_string(),
                                kind: ParamType::Uint(256),
                                internal_type: None,
                            },
                        ],
                        outputs: Vec::new(),
                        constant: None,
                        state_mutability: StateMutability::NonPayable,
                    }],
                )]),
                events: BTreeMap::new(),
                errors: BTreeMap::new(),
                receive: false,
                fallback: false,
            };
            let tokens = &[
                Token::FixedBytes(
                    deps.api
                        .addr_canonicalize(info.sender.as_str())?
                        .as_slice()
                        .to_vec(),
                ),
                Token::Address(Address::from_str(burn_info.recipient.as_str()).unwrap()),
                Token::Uint(Uint::from_big_endian(&burn_info.amount.to_be_bytes())),
                Token::Uint(Uint::from_big_endian(&nonce.to_be_bytes())),
            ];
            WITHDRAW_LIST.update(
                deps.storage,
                nonce,
                |burn_info| -> Result<_, ContractError> {
                    let mut burn_info = burn_info.unwrap();
                    burn_info.timestamp = env.block.time;
                    Ok(burn_info)
                },
            )?;
            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                    execute_job: ExecuteJob {
                        job_id: CHAIN_SETTINGS
                            .load(deps.storage, burn_info.chain_id.clone())?
                            .job_id,
                        payload: Binary::new(
                            contract
                                .function("withdraw")
                                .unwrap()
                                .encode_input(tokens.as_slice())
                                .unwrap(),
                        ),
                    },
                }))
                .add_attributes(vec![
                    ("action", "re_withdraw"),
                    ("chain_id", &burn_info.chain_id),
                    ("recipient", burn_info.recipient.as_str()),
                    ("nonce", &nonce.to_string()),
                ]))
        }
        ExecuteMsg::CancelWithdraw { nonce } => {
            // ACTION: Implement CancelWithdraw
            let burn_info = WITHDRAW_LIST.load(deps.storage, nonce)?;
            assert!(burn_info.burner == info.sender, "Unauthorized");
            // assert!(!burn_info.burned, "Already burned");
            assert!(
                burn_info
                    .timestamp
                    .plus_seconds(STATE.load(deps.storage)?.retry_delay)
                    > env.block.time,
                "Withdraw is pending"
            );
            WITHDRAW_LIST.remove(deps.storage, nonce);
            Ok(Response::new()
                .add_message(CosmosMsg::Bank(BankMsg::Send {
                    to_address: info.sender.to_string(),
                    amount: vec![Coin {
                        denom: STATE.load(deps.storage)?.denom,
                        amount: Uint128::from(burn_info.amount),
                    }],
                }))
                .add_attributes(vec![
                    ("action", "cancel_withdraw"),
                    ("nonce", &nonce.to_string()),
                    ("chain_id", &burn_info.chain_id),
                    ("recipient", burn_info.recipient.as_str()),
                    ("burner", burn_info.burner.as_str()),
                    ("amount", &burn_info.amount.to_string()),
                ]))
        }
        ExecuteMsg::UpdateConfig { retry_delay, owner } => {
            // ACTION: Implement UpdateConfig
            assert!(
                info.sender == STATE.load(deps.storage)?.owner,
                "Unauthorized"
            );
            STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
                if let Some(retry_delay) = retry_delay {
                    assert!(retry_delay > 0, "Retry delay must be greater than 0");
                    state.retry_delay = retry_delay;
                }
                if let Some(new_owner) = owner.clone() {
                    state.owner = new_owner;
                }
                Ok(state)
            })?;
            let mut attributes = vec![("action", "update_config")];
            let retry_delay_string: String;
            if retry_delay.is_some() {
                retry_delay_string = retry_delay.unwrap().to_string();
                attributes.push(("retry_delay", retry_delay_string.as_str()));
            }
            let owner_string: String;
            if owner.is_some() {
                owner_string = owner.unwrap().to_string();
                attributes.push(("owner", owner_string.as_str()));
            }
            Ok(Response::new().add_attributes(attributes))
        }

        ExecuteMsg::SetPaloma { chain_id } => {
            // ACTION: Implement SetPaloma
            let state = STATE.load(deps.storage)?;
            assert!(info.sender == state.owner, "Unauthorized");

            #[allow(deprecated)]
            let contract: Contract = Contract {
                constructor: None,
                functions: BTreeMap::from_iter(vec![(
                    "set_paloma".to_string(),
                    vec![Function {
                        name: "set_paloma".to_string(),
                        inputs: vec![],
                        outputs: Vec::new(),
                        constant: None,
                        state_mutability: StateMutability::NonPayable,
                    }],
                )]),
                events: BTreeMap::new(),
                errors: BTreeMap::new(),
                receive: false,
                fallback: false,
            };
            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                    execute_job: ExecuteJob {
                        job_id: CHAIN_SETTINGS.load(deps.storage, chain_id.clone())?.job_id,
                        payload: Binary::new(
                            contract
                                .function("set_paloma")
                                .unwrap()
                                .encode_input(&[])
                                .unwrap(),
                        ),
                    },
                }))
                .add_attribute("action", "set_paloma"))
        }
        ExecuteMsg::UpdateCompass {
            chain_id,
            new_compass,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(info.sender == state.owner, "Unauthorized");

            #[allow(deprecated)]
            let contract: Contract = Contract {
                constructor: None,
                functions: BTreeMap::from_iter(vec![(
                    "update_compass".to_string(),
                    vec![Function {
                        name: "update_compass".to_string(),
                        inputs: vec![Param {
                            name: "new_compass".to_string(),
                            kind: ParamType::Address,
                            internal_type: None,
                        }],
                        outputs: Vec::new(),
                        constant: None,
                        state_mutability: StateMutability::NonPayable,
                    }],
                )]),
                events: BTreeMap::new(),
                errors: BTreeMap::new(),
                receive: false,
                fallback: false,
            };
            let tokens = &[Token::Address(
                Address::from_str(new_compass.as_str()).unwrap(),
            )];
            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                    execute_job: ExecuteJob {
                        job_id: CHAIN_SETTINGS.load(deps.storage, chain_id.clone())?.job_id,
                        payload: Binary::new(
                            contract
                                .function("update_compass")
                                .unwrap()
                                .encode_input(tokens)
                                .unwrap(),
                        ),
                    },
                }))
                .add_attributes(vec![
                    ("action", "update_compass"),
                    ("chain_id", &chain_id),
                    ("new_compass", new_compass.as_str()),
                ]))
        }
        ExecuteMsg::UpdateRefundWallet {
            chain_id,
            new_refund_wallet,
        } => {
            let state = STATE.load(deps.storage)?;
            assert!(state.owner == info.sender, "Unauthorized");
            let update_refund_wallet_address: Address =
                Address::from_str(new_refund_wallet.as_str()).unwrap();
            #[allow(deprecated)]
            let contract: Contract = Contract {
                constructor: None,
                functions: BTreeMap::from_iter(vec![(
                    "update_refund_wallet".to_string(),
                    vec![Function {
                        name: "update_refund_wallet".to_string(),
                        inputs: vec![Param {
                            name: "new_refund_wallet".to_string(),
                            kind: ParamType::Address,
                            internal_type: None,
                        }],
                        outputs: Vec::new(),
                        constant: None,
                        state_mutability: StateMutability::NonPayable,
                    }],
                )]),
                events: BTreeMap::new(),
                errors: BTreeMap::new(),
                receive: false,
                fallback: false,
            };
            Ok(Response::new()
                .add_message(CosmosMsg::Custom(PalomaMsg::SchedulerMsg {
                    execute_job: ExecuteJob {
                        job_id: CHAIN_SETTINGS.load(deps.storage, chain_id.clone())?.job_id,
                        payload: Binary::new(
                            contract
                                .function("update_refund_wallet")
                                .unwrap()
                                .encode_input(&[Token::Address(update_refund_wallet_address)])
                                .unwrap(),
                        ),
                    },
                }))
                .add_attribute("action", "update_refund_wallet"))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetState {} => to_json_binary(&STATE.load(deps.storage)?),
        QueryMsg::GetWithdrawList {} => {
            let mut withdraw_list: Vec<(u64, BurnInfo)> = Vec::new();
            WITHDRAW_LIST
                .range(deps.storage, None, None, Order::Ascending)
                .for_each(|item| {
                    withdraw_list.push(item.unwrap());
                });
            to_json_binary(&withdraw_list)
        }
        QueryMsg::GetBurnInfo { nonce } => {
            to_json_binary(&WITHDRAW_LIST.load(deps.storage, nonce)?)
        }
        QueryMsg::GetChainSettings {} => {
            let mut chain_setting_info: Vec<ChainSettingInfo> = Vec::new();
            CHAIN_SETTINGS
                .range(deps.storage, None, None, Order::Ascending)
                .for_each(|item| {
                    let item = item.unwrap();
                    chain_setting_info.push(ChainSettingInfo {
                        chain_id: item.clone().0,
                        job_id: item.1.job_id.clone(),
                        minimum_amount: item.1.minimum_amount,
                    });
                });
            to_json_binary(&chain_setting_info)
        }
        QueryMsg::GetJobId { chain_id } => {
            to_json_binary(&CHAIN_SETTINGS.load(deps.storage, chain_id)?)
        }
        QueryMsg::ReWithdrawable {} => to_json_binary(&!WITHDRAW_LIST.is_empty(deps.storage)),
    }
}
