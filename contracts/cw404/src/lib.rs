pub mod contract;
mod error;
pub mod helpers;
pub mod msg;
pub mod state;

use cosmwasm_std::Empty;

pub use crate::error::ContractError;


// This is a simple type to let us handle empty extensions
pub type Extension = Option<Empty>;

// Version info for migration
pub const CONTRACT_NAME: &str = "crates.io:cw404";
pub const CONTRACT_VERSION: &str = "0.1.0";

pub mod entry {
    use self::{msg::{ContractInfoResponse, ExecuteMsg, InstantiateMsg, QueryMsg}, state::{ContractInfo, CONTRACT_INFO}};

    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    // This makes a conscious choice on the various generics used by the contract
    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        contract::instantiate(deps, env, info, msg)

        

        // let tract = Cw721Contract::<Extension, Empty, Empty, Empty>::default();
        // tract.instantiate(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Ok(Response::new())
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        to_json_binary("")
    }

    // #[cfg_attr(not(feature = "library"), entry_point)]
    // pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    //     // make sure the correct contract is being upgraded, and it's being
    //     // upgraded from the correct version.
    //     cw2::assert_contract_version(deps.as_ref().storage, CONTRACT_NAME, EXPECTED_FROM_VERSION)?;

    //     // update contract version
    //     cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    //     // perform the upgrade
    //     upgrades::v0_17::migrate::<Extension, Empty, Empty, Empty>(deps)
    // }
}
