use cosmwasm_std::{Deps, StdResult};

use crate::state::load_state;

use services::lending::StateResponse;

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
