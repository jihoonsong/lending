use cosmwasm_std::{Deps, StdResult};

use crate::state::{load_borrow_request, load_borrow_request_id_to_addr, load_state};

use services::lending::{BorrowRequestResponse, StateResponse};

/// ## Description
/// Returns lending contract state in the [`StateResponse`] object
/// ## Params
/// * **deps** is an object of type [`Deps`]
pub fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state = load_state(deps.storage)?;

    let res = StateResponse {
        request_count: state.request_count,
        response_count: state.response_count,
    };

    Ok(res)
}

/// ## Description
/// Returns borrow request by id in the [`BorrowRequestResponse`] object
/// ## Params
/// * **deps** is an object of type [`Deps`]
///
/// * **id** is [`u64`]
pub fn query_borrow_request_by_id(deps: Deps, id: u64) -> StdResult<BorrowRequestResponse> {
    let borrower = load_borrow_request_id_to_addr(deps.storage, id)?;
    let borrow_requests = load_borrow_request(deps.storage, &borrower)?;
    let borrow_request = borrow_requests.iter().find(|br| br.id == id).unwrap();

    let res = BorrowRequestResponse {
        id: borrow_request.id,
        collateral: borrow_request.collateral.clone(),
        period: borrow_request.period,
        borrowed_from: borrow_request.borrowed_from,
        borrowed_at: borrow_request.borrowed_at,
    };

    Ok(res)
}

/// ## Description
/// Returns a list of borrow requests by addr in the [`Vec<BorrowRequestResponse>`] object
/// ## Params
/// * **deps** is an object of type [`Deps`]
///
/// * **borrower** is [`String`]
pub fn query_borrow_requests_by_addr(
    deps: Deps,
    borrower: String,
) -> StdResult<Vec<BorrowRequestResponse>> {
    let borrower_raw = deps.api.addr_canonicalize(borrower.as_str())?;

    let res = load_borrow_request(deps.storage, &borrower_raw)?
        .into_iter()
        .map(|br| BorrowRequestResponse {
            id: br.id,
            collateral: br.collateral,
            period: br.period,
            borrowed_from: br.borrowed_from,
            borrowed_at: br.borrowed_at,
        })
        .collect();

    Ok(res)
}
