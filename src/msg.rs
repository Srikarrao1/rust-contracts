use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};
use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;

#[cw_serde]
// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub donation_denom: String,
}
#[cw_serde]
// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(rename_all = "snake_case")]
pub struct GreetResp {
    pub message: String,
}

#[cw_serde]
// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(rename_all = "snake_case")]
pub struct AdminsListResp {
    pub admins: Vec<Addr>,
}

#[cw_serde]
#[derive(QueryResponses)]
// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    #[returns(GreetResp)]
    Greet {},
    #[returns(AdminsListResp)]
    AdminsList {},
}

#[cw_serde]

// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// #[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddMembers { admins: Vec<String> },
    Leave {},
    Donate {},
}
