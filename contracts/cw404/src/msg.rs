use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_native_supply: u128,
    pub owner: String,

}

#[cw_serde]
pub enum ExecuteMsg {
    TransferOwnership {
        new_owner: String
    },
    RevokeOwnership {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct NftReceiveMsg {
    pub sender: String,
    pub from: String,
    pub token_id: u64,
    pub data: Binary,
}