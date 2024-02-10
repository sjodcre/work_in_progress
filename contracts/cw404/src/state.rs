use cosmwasm_std::{Addr, CustomMsg, Decimal, StdError, StdResult, Storage, Uint256, Uint64};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::msg::ContractInfoResponse;

pub struct OwnershipTransferredEvent {
    pub previous_owner: Addr,
    pub new_owner: Addr,
}


pub trait Ownable {
    
    fn is_owner(&self, sender: &Addr) -> bool;
    fn transfer_ownership(&mut self, storage: &mut dyn Storage, new_owner: Addr) -> StdResult<()>;
    fn revoke_ownership(&mut self, storage: &mut dyn Storage) -> StdResult<()>;
}

pub struct CW404<'a>
{
    pub owner: Addr,
    pub contract_info: Item<'a, ContractInfoResponse>,
    /// Stored as (granter, operator) giving operator full control over granter's account
    pub token_count: Item<'a, u64>,
    // pub operators: Map<'a, (&'a Addr, &'a Addr), Expiration>,
    // pub tokens: IndexedMap<'a, &'a str, TokenInfo<T>, TokenIndexes<'a, T>>,

}

impl <'a> Ownable for CW404<'a> {
    fn is_owner(&self, sender: &Addr) -> bool {
        &self.owner == sender
    }

    fn transfer_ownership(&mut self, storage: &mut dyn Storage, new_owner: Addr) -> StdResult<()> {
        if new_owner == Addr::unchecked("") {
            return Err(StdError::generic_err("InvalidOwner"));
        }
        self.owner = new_owner;
        Ok(())
    }

    fn revoke_ownership(&mut self, storage: &mut dyn Storage) -> StdResult<()> {
        self.owner = Addr::unchecked("");
        Ok(())
    }
}


// Storage for the contract's state
// pub const contract_info: Item<'a, ContractInfoResponse>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
}



pub const CONTRACT_INFO: Item<ContractInfo> = Item::new("contract_info");
pub const MINTED: Item<u64> = Item::new("minted");
// Mappings translated to cw-storage-plus Maps
// balance of user in fractional representation
pub const BALANCE: Map<&Addr, u128> = Map::new("balance");
//allowance of user in fractional representation
pub const ALLOWANCE: Map<(&Addr, &Addr), u128> = Map::new("allowance");
//approval in native representation
pub const APPROVALS: Map<u128, Addr> = Map::new("approvals");
//approval for all in native representation
pub const APPROVALS_FOR_ALL: Map<(&Addr, &Addr), bool> = Map::new("approvals_for_all");
//array of owned ids in native representation
pub const OWNED: Map<&Addr, Vec<u128>> = Map::new("owned");
//track indices for the _owned mapping
pub const OWNED_INDEX: Map<u128, u128> = Map::new("owned_index");
// Addresses whitelisted from minting / burning for gas savings (pairs, routers, etc)
pub const WHITELIST: Map<&str, bool> = Map::new("whitelist");