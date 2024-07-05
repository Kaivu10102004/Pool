use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub deposit_token : Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const DEPOSIT_NATIVE_TOKEN : Map<Addr,Uint128> = Map::new("deposit_native_token");
pub const DEPOSIT_TOKEN : Map<Addr,Uint128> = Map::new("deposit_token");

