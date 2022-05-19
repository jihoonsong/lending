use astroport::asset::Asset;
use cosmwasm_std::{Attribute, CanonicalAddr, DepsMut, Response};

use crate::{
    error::ContractError,
    state::{
        load_borrow_request, load_state, store_borrow_request, store_borrow_request_id_to_addr,
        store_state, BorrowRequest,
    },
};

/// ## Description
/// Make borrow request
/// Returns [`Response`] with specified attributes and messages if operation was successful
/// Otherwise returns [`ContractError`]
/// ## Params
/// * **deps** is an object of type [`DepsMut`]
///
/// * **borrower** is an object of type [`CanonicalAddr`]
///
/// * **collateral** is an object of type [`Asset`]
///
/// * **period** is [`u64`]
pub fn make_borrow_request(
    deps: DepsMut,
    borrower: &CanonicalAddr,
    collateral: Asset,
    period: u64,
) -> Result<Response, ContractError> {
    if collateral.amount.is_zero() {
        return Err(ContractError::InvalidAmount {});
    }

    if period == 0u64 {
        return Err(ContractError::InvalidPeriod {});
    }

    let mut attrs: Vec<Attribute> = vec![Attribute::new("action", "make_borrow_request")];

    let mut state = load_state(deps.storage)?;
    state.request_count = state.request_count.checked_add(1u64).unwrap();
    store_state(deps.storage, &state)?;
    attrs.push(Attribute::new(
        "request_count",
        state.request_count.to_string(),
    ));

    let id = state.request_count;

    let mut borrow_request = load_borrow_request(deps.storage, borrower)?;
    borrow_request.push(BorrowRequest {
        id,
        collateral,
        period,
        borrowed_from: 0u64,
        borrowed_at: 0u64,
    });
    store_borrow_request(deps.storage, &borrower, &borrow_request)?;
    store_borrow_request_id_to_addr(deps.storage, id, borrower)?;
    attrs.push(Attribute::new(
        "borrow_request",
        &format!(
            "\
            id: {},\n\
            collateral: {},\n\
            period: {},\n\
            borrowed_from: 0,\n\
            borrowed_at: 0,\n\
            ",
            id,
            &borrow_request.last().unwrap().collateral,
            period,
        ),
    ));

    Ok(Response::new().add_attributes(attrs))
}
