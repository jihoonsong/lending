#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, from_binary, to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};
use cw2::set_contract_version;

use crate::{
    commands,
    error::ContractError,
    queries,
    state::{store_state, State},
    utils::{to_cw20_token, to_native_token},
};

use services::lending::{Cw20HookMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

/// Contract name that is used for migration.
const CONTRACT_NAME: &str = "jihoonsong-lending";
/// Contract version that is used for migration.
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    store_state(deps.storage, &State::default())?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::MakeBorrowRequestCw20Token(cw20_receive_msg) => {
            match from_binary(&cw20_receive_msg.msg) {
                Ok(Cw20HookMsg::MakeBorrowRequest { period }) => {
                    let borrower = deps
                        .api
                        .addr_canonicalize(cw20_receive_msg.sender.as_str())?;
                    let collateral = to_cw20_token(info.sender, cw20_receive_msg.amount)?;
                    commands::make_borrow_request(deps, &borrower, collateral, period)
                }
                Err(_) => Err(ContractError::InvalidCw20HookMsg {}),
            }
        }
        ExecuteMsg::MakeBorrowRequestNativeToken { period } => {
            validate_funds(&info.funds)?;
            let borrower = deps.api.addr_canonicalize(info.sender.as_str())?;
            let collateral = to_native_token(info.funds[0].denom.clone(), info.funds[0].amount)?;
            commands::make_borrow_request(deps, &borrower, collateral, period)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::State {} => to_binary(&queries::query_state(deps)?),
        QueryMsg::BorrowRequestById { id } => {
            to_binary(&queries::query_borrow_request_by_id(deps, id)?)
        }
        QueryMsg::BorrowRequestByAddr { borrower } => {
            to_binary(&queries::query_borrow_requests_by_addr(deps, borrower)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

fn validate_funds(funds: &Vec<Coin>) -> Result<(), ContractError> {
    if funds.len() != 1 {
        return Err(ContractError::InvalidFunds {});
    }

    if funds[0].amount.is_zero() {
        return Err(ContractError::InvalidAmount {});
    }

    Ok(())
}
