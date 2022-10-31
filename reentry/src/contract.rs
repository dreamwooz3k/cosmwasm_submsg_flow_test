#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, CosmosMsg, WasmMsg};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, TargetMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:reentry";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg
    {
        ExecuteMsg::Attack { submsg_addr } => execute_attack(info, submsg_addr),
    }
}

fn execute_attack(info: MessageInfo, _submsg_addr: String) -> Result<Response, ContractError>
{
    let msg1 = to_binary(&TargetMsg::Flow{ submsg_addr:_submsg_addr })?;
    Ok(Response::new().add_message(
        CosmosMsg::Wasm(WasmMsg::Execute{
            contract_addr: info.sender.clone().to_string(),
            msg: msg1,
            funds: vec![],
        })
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
