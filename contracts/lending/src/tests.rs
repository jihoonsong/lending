use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info},
};

use crate::contract::{instantiate, query};

use services::lending::{InstantiateMsg, QueryMsg, StateResponse};

/// Mock address 0 used for test.
const MOCK_ADDRESS_0: &str = "mock_address_0";

/// ## Description
/// Test if instantiation works properly
/// ## Test case 0
/// Input: Instantiate contract
/// Output: StateResponse struct with zero values
#[test]
fn instantiate_contract() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info(MOCK_ADDRESS_0, &[]);
    let msg = InstantiateMsg {};

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(
        from_binary::<StateResponse>(
            &query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap()
        )
        .unwrap(),
        StateResponse {
            request_count: 0u64,
            response_count: 0u64,
        },
    );
}
