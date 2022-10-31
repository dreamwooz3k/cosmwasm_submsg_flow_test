use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Test1 {},
    Test2 {},
    Test3 {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
