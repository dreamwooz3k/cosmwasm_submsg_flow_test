use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Attack { submsg_addr: String }
}

#[cw_serde]
pub enum TargetMsg {
    Flow { submsg_addr: String }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}