// #[cfg(not(feature = "library"))]
// use cosmwasm_std::entry_point;
// use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// // use cw2::set_contract_version;

// use crate::error::ContractError;
// use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
// use crate::state::{Config, State, CONFIG, OWNED, STATE, WHITELIST};

// /*
// // version info for migration info
// const CONTRACT_NAME: &str = "crates.io:cw404";
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
// */

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn instantiate(
//     deps: DepsMut,
//     _env: Env,
//     info: MessageInfo,
//     msg: InstantiateMsg,
// ) -> Result<Response, ContractError> {
//     let config = Config {
//         name: msg.name,
//         symbol: msg.symbol,
//         decimals: msg.decimals,
//         total_supply: msg.total_native_supply * (10u128.pow(msg.decimals as u32)),
//         minted: 0,
//     };
//     CONFIG.save(deps.storage, &config)?;
    
//     let state = State {
//         owner: info.sender,
//     };
//     STATE.save(deps.storage, &state)?;
    
//     Ok(Response::new().add_attribute("method", "instantiate"))
// }

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn execute(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     msg: ExecuteMsg,
// ) -> Result<Response, ContractError> {
//     match msg {
//         ExecuteMsg::TransferOwnership { new_owner } => {
//             execute_transfer_ownership(deps, env, info, new_owner)
//         }
//         ExecuteMsg::RevokeOwnership {} => execute_revoke_ownership(deps, env, info),
//     }
// }

// pub fn set_whitelist(deps: DepsMut, info: MessageInfo, target: String, state: bool) -> Result<Response, ContractError> {
//     // Check if the sender is the owner
//     let owner = STATE.load(deps.storage)?.owner;
//     if info.sender != owner {
//         return Err(ContractError::Unauthorized{});
//     }

//     // Update the whitelist state
//     WHITELIST.save(deps.storage, &target, &state)?;

//     Ok(Response::new().add_attribute("action", "set_whitelist").add_attribute("target", target).add_attribute("state", state.to_string()))
// }

// fn receive_nft(
//     deps: Deps,
//     sender: String,
//     from: String,
//     token_id: u64,
//     data: Binary,
// ) -> StdResult<Response> {
//     // Logic to handle the received NFT, e.g., storing metadata, updating state, etc.
//     // For this example, we'll simply return an Ok response with some attributes for demonstration

//     Ok(Response::new()
//         .add_attribute("action", "receive_nft")
//         .add_attribute("sender", sender)
//         .add_attribute("from", from)
//         .add_attribute("token_id", token_id.to_string()))
// }

// fn execute_transfer_ownership(
//     deps: DepsMut,
//     _env: Env,
//     info: MessageInfo,
//     new_owner: String,
// ) -> Result<Response, ContractError> {
//     let mut state = STATE.load(deps.storage)?;
//     if info.sender != state.owner {
//         return Err(ContractError::Unauthorized {});
//     }
//     if new_owner.trim().is_empty() {
//         return Err(ContractError::Generic {});
//     }
//     let new_owner_addr = deps.api.addr_validate(&new_owner)?;
//     state.owner = new_owner_addr;
//     STATE.save(deps.storage, &state)?;
//     Ok(Response::new().add_attribute("action", "transfer_ownership"))
// }

// fn execute_revoke_ownership(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
//     let mut state = STATE.load(deps.storage)?;
//     if info.sender != state.owner {
//         return Err(ContractError::Unauthorized{});
//     }
//     state.owner = Addr::unchecked(""); // Effectively removes ownership
//     STATE.save(deps.storage, &state)?;
//     Ok(Response::new().add_attribute("action", "revoke_ownership"))
// }

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
//     unimplemented!()
// }

// pub fn query_owner_of(deps: Deps, token_id: u128) -> StdResult<String> {
//     let owner = OWNED.load(deps.storage, &token_id.to_string())?;
//     let s: String = owner.into_iter().collect();
//     Ok(s)
// }

// #[cfg(test)]
// mod tests {}

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:clean";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
