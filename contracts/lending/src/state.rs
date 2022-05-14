use cosmwasm_std::{StdResult, Storage};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// ## Description
/// Stores a struct of type [`State`] at the given key
static STATE: Item<State> = Item::new("state");

/// ## Description
/// This structure describes state of lending contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    /// The number of requests has been made
    pub request_count: u64,
    /// The number of responses has been made
    pub response_count: u64,
}

impl Default for State {
    fn default() -> Self {
        State {
            request_count: 0u64,
            response_count: 0u64,
        }
    }
}

/// ## Description
/// Saves changes of [`State`] struct in [`STATE`] storage
/// ## Params
/// * **storage** is an object of type [`Storage`]
///
/// * **state** is a struct of type [`State`] to be stored
pub fn store_state(storage: &mut dyn Storage, state: &State) -> StdResult<()> {
    STATE.save(storage, state)
}

/// ## Description
/// Returns struct of type [`State`]
/// ## Params
/// * **storage** is an object of type [`Storage`]
pub fn load_state(storage: &dyn Storage) -> StdResult<State> {
    STATE.load(storage)
}
