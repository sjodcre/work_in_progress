// use cosmwasm_std::{Addr, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
// use cw_storage_plus::{Item, Map};
// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct State {
//     pub owner: Addr,
// }

// // Storage for the contract's state
// pub const STATE: Item<State> = Item::new("state");


// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Config {
//     pub name: String,
//     pub symbol: String,
//     pub decimals: u8,
//     pub total_supply: u128,
//     pub minted: u128,
// }

// pub const CONFIG: Item<Config> = Item::new("config");

// // Mappings translated to cw-storage-plus Maps
// pub const BALANCES: Map<&str, u128> = Map::new("balances");
// pub const ALLOWANCES: Map<(&str, &str), u128> = Map::new("allowances");
// pub const APPROVALS: Map<u128, String> = Map::new("approvals");
// pub const APPROVALS_FOR_ALL: Map<(&str, &str), bool> = Map::new("approvals_for_all");
// pub const OWNED: Map<&str, Vec<u128>> = Map::new("owned");
// pub const OWNED_INDEX: Map<u128, u128> = Map::new("owned_index");
// pub const WHITELIST: Map<&str, bool> = Map::new("whitelist");