use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("Invalid Cw20HookMsg")]
    InvalidCw20HookMsg {},

    #[error("Invalid funds")]
    InvalidFunds {},

    #[error("Invalid amount")]
    InvalidAmount {},

    #[error("Invalid period")]
    InvalidPeriod {},
}
