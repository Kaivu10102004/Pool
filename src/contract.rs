use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg,MigrateMsg};
use crate::state::{Config, CONFIG,DEPOSIT, Asset, AssetInfo};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr,
    to_json_binary, BankMsg, Coin, DepsMut, Env, MessageInfo, Response, Uint128, WasmMsg,Deps,StdResult,Binary,
};

use cw2::{set_contract_version};
use cw20::{self, Cw20ExecuteMsg};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(
        deps.storage,
        &Config {
            owner: msg.owner,
        },
    )?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit{assets} => execute_deposit(deps,env,info, assets),
        ExecuteMsg::WithDraw{denom, token} => execute_with_draw(deps,env,info,token,denom),
    }
}

pub fn execute_deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    assets : Vec<Asset>,
) -> Result<Response, ContractError>{
    for asset in assets {
        match asset.info {
            AssetInfo::Token{contract_addr} => {
                let deposit_token_info = DEPOSIT.may_load(deps.storage,(info.sender.clone(),contract_addr.to_string()))?;
                match deposit_token_info {
                    Some(total_token_amount) => { 
                        DEPOSIT.save(deps.storage, (info.sender.clone(),contract_addr.to_string()), &(total_token_amount.clone() + &asset.amount.clone()))?;
                    }
                    None => {
                        DEPOSIT.save(deps.storage, (info.sender.clone(),contract_addr.to_string().clone()), &asset.amount.clone())?;
                        }
                }
                let transfer_msg = WasmMsg::Execute {
                    contract_addr: contract_addr.to_string(),
                    msg: to_json_binary(&Cw20ExecuteMsg::TransferFrom {
                        owner : info.sender.clone().to_string(),
                        recipient: env.contract.address.to_string(),
                        amount: asset.amount,
                    })?,
                    funds: vec![],
                };
                //Ok(Response::default().add_message(transfer_msg))
            }
            AssetInfo::NativeToken { denom } => {
                let deposit_token_info = DEPOSIT.may_load(deps.storage,(info.sender.clone(),denom.clone()))?;
                match deposit_token_info {
                    Some(total_token_amount) => { 
                        DEPOSIT.save(deps.storage, (info.sender.clone(),denom), &(total_token_amount.clone() + asset.amount.clone()))?;
                    }
                    None => {
                        DEPOSIT.save(deps.storage, (info.sender.clone(),denom), &asset.amount.clone())?;
                    }
                }
            }
        }
    }
    
    Ok(Response::default())
}


pub fn execute_with_draw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    token : Option<Addr>,
    denom : Option<String>,
) -> Result<Response, ContractError>{
    if denom.is_some() {
        let denom = denom.expect("Not found denom");
        let with_draw_native_token_info = DEPOSIT.may_load(deps.storage,(info.sender.clone(),denom.clone()))?;
        match with_draw_native_token_info {
            Some(total_native_token_amount) => { 
                DEPOSIT.save(deps.storage, (info.sender.clone(),denom.clone()), &Uint128::zero())?;
                let transfer_msg = BankMsg::Send {
                    to_address: info.sender.to_string(),
                    amount: vec![Coin {
                        denom : denom,
                        amount : total_native_token_amount,
                    }],
                };
                Ok(Response::default().add_message(transfer_msg))
            }
            None => Err(ContractError::NoStakerInfo {}),
        }
    }
    else if token.is_some() {
        let token = token.expect("Not found denom");
        let with_draw_token_info = DEPOSIT.may_load(deps.storage,(info.sender.clone(),token.to_string().clone()))?;
        match with_draw_token_info {
            Some(total_token_amount) => { 
                DEPOSIT.save(deps.storage, (info.sender.clone(),token.to_string().clone()), &Uint128::zero())?;
                let transfer_msg = WasmMsg::Execute {
                    contract_addr: token.to_string().clone(),
                    msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                        recipient: info.sender.clone().to_string(),
                        amount: Uint128::from(total_token_amount.clone()),
                    })?,
                    funds: vec![],
                };
                Ok(Response::default().add_message(transfer_msg))
            }
            None => Err(ContractError::NoStakerInfo {}),
        }
    }
    else {
        Err(ContractError::NoStakerInfo {})
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps, 
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::NativeTokenInfo { user , denom} => {
            let deposit_info = DEPOSIT.may_load(deps.storage, (user, denom))?;
            match deposit_info {
                    Some(deposit_info) => to_json_binary(&deposit_info),
                    None => to_json_binary(&Uint128::zero())
            }
        }
        QueryMsg::TokenInfo {user, token} => { 
            let deposit_info = DEPOSIT.may_load(deps.storage, (user,token.to_string()))?;
            match deposit_info {
                Some(deposit_info) => to_json_binary(&deposit_info),
                None => to_json_binary(&Uint128::zero())
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    //_info: MessageInfo,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}


#[cfg(test)]
mod tests {}
