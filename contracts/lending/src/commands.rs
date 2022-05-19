use astroport::asset::Asset;
use cosmwasm_std::{CanonicalAddr, DepsMut, Response};

use crate::error::ContractError;

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
    _deps: DepsMut,
    _borrower: &CanonicalAddr,
    _collateral: Asset,
    _period: u64,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}
