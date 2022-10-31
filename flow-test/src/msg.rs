use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Flow { submsg_addr: String },
    Flow2 { submsg_addr: String, reentry_addr: String }
}

#[cw_serde]
pub enum TargetMsg {
    Test1 {},
    Test2 {},
    Test3 {},
    Test1231321 {},
    Attack { submsg_addr: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
