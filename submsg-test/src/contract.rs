#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, self};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:submsg-test";
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
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg
    {
        ExecuteMsg::Test1 {} => execute_test1(),
        ExecuteMsg::Test2 {} => execute_test2(),
        ExecuteMsg::Test3 {} => execute_test3()
    }
}

pub fn execute_test1() -> Result<Response, ContractError>
{
    let res = Response::new()
        .add_attribute("action", "test1");
    Ok(res)
}

pub fn execute_test2() -> Result<Response, ContractError>
{
    let res = Response::new()
        .add_attribute("action", "test2");
    Ok(res)
}

pub fn execute_test3() -> Result<Response, ContractError>
{
    let res = Response::new()
        .add_attribute("action", "test3");
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
