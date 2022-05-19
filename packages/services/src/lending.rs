use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

/// ## ExecuteMsg
/// This structure describes execute messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// ## Description
    /// Make borrow request with CW20 token
    MakeBorrowRequestCw20Token(Cw20ReceiveMsg),
    /// ## Description
    /// Make borrow request with native token
    MakeBorrowRequestNativeToken { period: u64 },
}

/// ## Cw20HookMsg
/// This structure describes the CW20 hook message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    /// ## Description
    /// Make borrow request
    MakeBorrowRequest { period: u64 },
}

/// ## Description
/// This structure describes query messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    State {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

/// ## Description
/// This structure describes state response message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    /// The number of requests has been made
    pub request_count: u64,
    /// The number of responses has been made
    pub response_count: u64,
}
