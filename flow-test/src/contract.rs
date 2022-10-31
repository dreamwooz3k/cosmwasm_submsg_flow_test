#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, CosmosMsg, WasmMsg, to_binary};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, TargetMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:flow-test";
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
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg
    {
        ExecuteMsg::Flow { submsg_addr } => execute_flow(submsg_addr),
        ExecuteMsg::Flow2 { submsg_addr, reentry_addr } => execute_flow2(env, submsg_addr, reentry_addr)
    }
}

pub fn execute_flow(_submsg_addr: String) -> Result<Response, ContractError>
{
    let msgs = to_binary(&TargetMsg::Test1{})?;
    Ok(Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute { contract_addr: _submsg_addr.clone(), msg: msgs, funds: vec![] })))
}

pub fn execute_flow2(env: Env, _submsg_addr: String, _reentry_addr: String) -> Result<Response, ContractError>
{
    let msgs1 = to_binary(&TargetMsg::Test1{})?;
    let msgs3 = to_binary(&TargetMsg::Test2{})?;
    let msgs2 = to_binary(&TargetMsg::Test3{})?;
    let msgs4 = to_binary(&ExecuteMsg::Flow{ submsg_addr: _submsg_addr.to_string()})?;
    let msgs5 = to_binary(&TargetMsg::Attack{})?;
    Ok(Response::new().add_messages(vec![
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: _submsg_addr.clone(),
            msg: msgs1, // test 1
            funds: vec![],
        }),
        CosmosMsg::Wasm(WasmMsg::Execute{
            contract_addr: _reentry_addr.clone(),
            msg: msgs5, // test 1
            funds: vec![],
        }),
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: _submsg_addr.clone(),
            msg: msgs2, // test3
            funds: vec![],
        }),
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: _submsg_addr.clone(),
            msg: msgs3, // test 2
            funds: vec![],
        }),
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: msgs4, // test 1
            funds: vec![],
        })
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
