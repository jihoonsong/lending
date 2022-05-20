use astroport::asset::Asset;
use cosmwasm_std::{CanonicalAddr, StdResult, Storage};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// ## Description
/// Stores a struct of type [`State`] at the given key
static STATE: Item<State> = Item::new("state");

/// ## Description
/// A map which stores borrow requests from borrowers with
/// [`CanonicalAddr`] type as key and [`Vec<BorrowRequest>`] type as value
static BORROW_REQUEST: Map<&[u8], Vec<BorrowRequest>> = Map::new("borrow_request");

/// ## Description
/// A map which stores id and borrower address of each borrow request
/// [`u64`] type as key and [`CanonicalAddr`] type as value
static BORROW_REQUEST_ID_TO_ADDR: Map<&str, CanonicalAddr> = Map::new("borrow_request_id_to_addr");

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
/// This structure describes borrow request from borrower
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BorrowRequest {
    // Id, which is start from 1
    pub id: u64,
    // Collateral asset
    pub collateral: Asset,
    // Borrowing block period
    pub period: u64,
    // An id of borrow response, which responding to this request
    pub borrowed_from: u64,
    // A block height that borrowing occured at
    pub borrowed_at: u64,
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

/// ## Description
/// Saves changes of vector of [`BorrowRequest`] struct in [`BORROW_REQUEST`] storage
/// ## Params
/// * **storage** is an object of type [`Storage`]
///
/// * **borrower** is an object of type [`CanonicalAddr`]
///
/// * **borrow_request** is a vector of struct of type [`BorrowRequest`] to be stored
pub fn store_borrow_request(
    storage: &mut dyn Storage,
    borrower: &CanonicalAddr,
    borrow_request: &Vec<BorrowRequest>,
) -> StdResult<()> {
    BORROW_REQUEST.save(storage, borrower.as_slice(), borrow_request)
}

/// ## Description
/// Returns a vector of struct of type [`BorrowRequest`]
/// ## Params
/// * **storage** is an object of type [`Storage`]
///
/// * **borrower** is an object of type [`CanonicalAddr`]
pub fn load_borrow_request(
    storage: &dyn Storage,
    borrower: &CanonicalAddr,
) -> StdResult<Vec<BorrowRequest>> {
    BORROW_REQUEST
        .may_load(storage, borrower.as_slice())
        .map(|res| res.unwrap_or_default())
}

/// ## Description
/// Saves changes of an object of type [`CanonicalAddr`] in [`BORROW_REQUEST_ID_TO_ADDR`] storage
/// ## Params
/// * **storage** is an object of type [`Storage`]
///
/// * **id** is [`u64`]
///
/// * **borrower** is an object of type [`CanonicalAddr`]
pub fn store_borrow_request_id_to_addr(
    storage: &mut dyn Storage,
    id: u64,
    borrower: &CanonicalAddr,
) -> StdResult<()> {
    BORROW_REQUEST_ID_TO_ADDR.save(storage, &id.to_string(), borrower)
}

/// ## Description
/// Returns an object of type [`CanonicalAddr`]
/// ## Params
/// * **storage** is an object of type [`Storage`]
///
/// * **id** is [`u64`]
pub fn load_borrow_request_id_to_addr(storage: &dyn Storage, id: u64) -> StdResult<CanonicalAddr> {
    BORROW_REQUEST_ID_TO_ADDR
        .may_load(storage, &id.to_string())
        .map(|res| res.unwrap())
}

/// ## Description
/// Remove an object of type [`CanonicalAddr`] in [`BORROW_REQUEST_ID_TO_ADDR`] storage
/// ## Params
/// * **storage** is an object of type [`Storage`]
///
/// * **id** is [`u64`]
pub fn remove_borrow_request_id_to_addr(storage: &mut dyn Storage, id: u64) {
    BORROW_REQUEST_ID_TO_ADDR.remove(storage, &id.to_string())
}
