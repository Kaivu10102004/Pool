use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg,MigrateMsg};
use crate::state::{Config, CONFIG,DEPOSIT};
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
            //deposit_token: msg.deposit_token,
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
        ExecuteMsg::Deposit{amount, token} => execute_deposit(deps,env,info, token,amount),
        //ExecuteMsg::DepositToken{amount} => execute_deposit_token(deps,env,info,amount),
        ExecuteMsg::WithDrawNativeToken {denom} => execute_with_draw_native_token(deps,env,info,denom),
        ExecuteMsg::WithDrawToken{token} => execute_with_draw_token(deps,env,info,token),
    }
    //return Ok(Response::default())
}

pub fn execute_deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token : Addr,
    amount : Uint128,
) -> Result<Response, ContractError>{
    let coin = &info.funds[0];
    //let config = CONFIG.load(deps.storage)?; 
    let deposit_native_token_info = DEPOSIT.may_load(deps.storage,(info.sender.clone(),coin.denom.clone()))?;
    match deposit_native_token_info {
        Some(total_token_amount) => { 
            DEPOSIT.save(deps.storage, (info.sender.clone(),coin.denom.clone()), &(total_token_amount.clone() + coin.amount.clone()))?;
        }
        None => {
            DEPOSIT.save(deps.storage, (info.sender.clone(),coin.denom.clone()), &coin.amount.clone())?;
        }
    }

    let deposit_token_info = DEPOSIT.may_load(deps.storage,(info.sender.clone(),token.to_string().clone()))?;
    match deposit_token_info {
        Some(total_token_amount) => { 
            DEPOSIT.save(deps.storage, (info.sender.clone(),token.to_string().clone()), &(total_token_amount.clone() + amount.clone()))?;
        }
        None => {
            DEPOSIT.save(deps.storage, (info.sender.clone(),token.to_string().clone()), &amount.clone())?;
            }
    }
    let transfer_msg = WasmMsg::Execute {
        contract_addr: token.to_string().clone(),
        msg: to_json_binary(&Cw20ExecuteMsg::TransferFrom {
            owner : info.sender.clone().to_string(),
            recipient: env.contract.address.to_string(),
            amount: Uint128::from(amount.clone()),
        })?,
        funds: vec![],
    };
    Ok(Response::default().add_message(transfer_msg))
}

// pub fn execute_deposit(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     amount : Option<Uint128>,
// ) -> Result<Response, ContractError>{
//     let config = CONFIG.load(deps.storage)?; 
//     let deposit_token_info = DEPOSIT.may_load(deps.storage,info.sender.clone())?;
//     match deposit_token_info {
//         Some(total_token_amount) => { 
//             DEPOSIT.save(deps.storage, info.sender.clone(), &(total_token_amount.clone() + amount.clone()))?;
//         }
//         None => {
//             DEPOSIT.save(deps.storage, info.sender.clone(), &amount.clone())?;
//         }
//     }
//     let transfer_msg = WasmMsg::Execute {
//                 contract_addr: config.deposit_token.clone().to_string(),
//                 msg: to_json_binary(&Cw20ExecuteMsg::TransferFrom {
//                     owner : info.sender.clone().to_string(),
//                     recipient: env.contract.address.to_string(),
//                     amount: Uint128::from(amount.clone()),
//                 })?,
//                 funds: vec![]Coin,
//             };
//     Ok(Response::default().add_message(transfer_msg))
// }

pub fn execute_with_draw_native_token(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    denom : String,
) -> Result<Response, ContractError>{
    //let config = CONFIG.load(deps.storage)?; 
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
pub fn execute_with_draw_token(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    token : Addr,
) -> Result<Response, ContractError>{
    //let config = CONFIG.load(deps.storage)?; 
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
    //Ok(Response::default().add_message(transfer_msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps, 
    _env: Env,
    msg: QueryMsg,
    //sender : Addr,
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
