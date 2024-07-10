use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    //pub deposit_token : Addr,
}
#[cw_serde]
pub struct Asset {
    pub info: AssetInfo,
    pub amount: Uint128,
}
#[cw_serde]
pub enum AssetInfo {
    Token { contract_addr: Addr },
    NativeToken { denom: String },
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const DEPOSIT : Map<(Addr,String),Uint128> = Map::new("deposit");
