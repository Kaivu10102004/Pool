use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
   // pub deposit_token: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    //DepositNativeToken{},
    Deposit{amount : Option<Uint128>, token : Option<Addr>},
    //WithDrawNativeToken {denom : String},
    WithDraw{token : Option<Addr>, denom : Option<String>},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Uint128)]
    NativeTokenInfo { user: Addr, denom : String},
    #[returns(Uint128)]
    TokenInfo { user: Addr, token : Addr },
}

#[cw_serde]
pub enum MigrateMsg{}