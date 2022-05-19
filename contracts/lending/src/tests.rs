use astroport::asset::{Asset, AssetInfo};
use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info},
    to_binary, Addr, Coin, Uint128,
};
use cw20::Cw20ReceiveMsg;

use crate::{
    contract::{execute, instantiate, query},
    error::ContractError,
};

use services::lending::{
    BorrowRequestResponse, Cw20HookMsg, ExecuteMsg, InstantiateMsg, QueryMsg, StateResponse,
};

/// Mock address 0 used for test.
const MOCK_ADDRESS_0: &str = "mock_address_0";
/// Mock CW20 token address 0 used for test.
const MOCK_CW20_TOKEN_0: &str = "cw20_token_0";
/// Mock native token address 0 used for test.
const MOCK_NATIVE_TOKEN_0: &str = "native_token_0";

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

/// ## Description
/// Test if making borrow request works properly
/// ## Test case 0
/// Input: CW20 token with positive amount and positive period
/// Output: BorrowRequest struct with the given CW20 token
#[test]
fn make_borrow_request_test_case_0() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info(MOCK_ADDRESS_0, &[]);
    let msg = InstantiateMsg {};

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let amount = Uint128::from(1000u128);
    let period = 17200u64;

    let info = mock_info(MOCK_CW20_TOKEN_0, &[]);
    let msg = ExecuteMsg::MakeBorrowRequestCw20Token(Cw20ReceiveMsg {
        sender: MOCK_ADDRESS_0.to_string(),
        amount,
        msg: to_binary(&Cw20HookMsg::MakeBorrowRequest { period }).unwrap(),
    });

    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(
        from_binary::<Vec<BorrowRequestResponse>>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::BorrowRequestByAddr {
                    borrower: MOCK_ADDRESS_0.to_string(),
                },
            )
            .unwrap(),
        )
        .unwrap(),
        vec![BorrowRequestResponse {
            id: 1u64,
            collateral: Asset {
                info: AssetInfo::Token {
                    contract_addr: Addr::unchecked(MOCK_CW20_TOKEN_0.to_string()),
                },
                amount,
            },
            period,
            borrowed_from: 0u64,
            borrowed_at: 0u64,
        }]
    );
}

/// ## Description
/// Test if making borrow request works properly
/// ## Test case 1
/// Input: CW20 token with zero amount
/// Output: Error
#[test]
fn make_borrow_request_test_case_1() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info(MOCK_ADDRESS_0, &[]);
    let msg = InstantiateMsg {};

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let amount = Uint128::from(0u128);
    let period = 17200u64;

    let info = mock_info(MOCK_CW20_TOKEN_0, &[]);
    let msg = ExecuteMsg::MakeBorrowRequestCw20Token(Cw20ReceiveMsg {
        sender: MOCK_ADDRESS_0.to_string(),
        amount,
        msg: to_binary(&Cw20HookMsg::MakeBorrowRequest { period }).unwrap(),
    });

    let res = execute(deps.as_mut(), mock_env(), info, msg);

    match res {
        Err(ContractError::InvalidAmount {}) => (),
        _ => panic!("DO NOT ENTER HERE"),
    }
}

/// ## Description
/// Test if making borrow request works properly
/// ## Test case 2
/// Input: CW20 token with zero period
/// Output: Error
#[test]
fn make_borrow_request_test_case_2() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info(MOCK_ADDRESS_0, &[]);
    let msg = InstantiateMsg {};

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let amount = Uint128::from(1000u128);
    let period = 0u64;

    let info = mock_info(MOCK_CW20_TOKEN_0, &[]);
    let msg = ExecuteMsg::MakeBorrowRequestCw20Token(Cw20ReceiveMsg {
        sender: MOCK_ADDRESS_0.to_string(),
        amount,
        msg: to_binary(&Cw20HookMsg::MakeBorrowRequest { period }).unwrap(),
    });

    let res = execute(deps.as_mut(), mock_env(), info, msg);

    match res {
        Err(ContractError::InvalidPeriod {}) => (),
        _ => panic!("DO NOT ENTER HERE"),
    }
}

/// ## Description
/// Test if making borrow request works properly
/// ## Test case 3
/// Input: Native token with positive amount and positive period
/// Output: BorrowRequest struct with the given native token
#[test]
fn make_borrow_request_test_case_3() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info(MOCK_ADDRESS_0, &[]);
    let msg = InstantiateMsg {};

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let amount = Uint128::from(1000u128);
    let period = 17200u64;

    let info = mock_info(
        MOCK_ADDRESS_0,
        &[Coin {
            denom: MOCK_NATIVE_TOKEN_0.to_string(),
            amount,
        }],
    );
    let msg = ExecuteMsg::MakeBorrowRequestNativeToken { period };

    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(
        from_binary::<Vec<BorrowRequestResponse>>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::BorrowRequestByAddr {
                    borrower: MOCK_ADDRESS_0.to_string(),
                },
            )
            .unwrap(),
        )
        .unwrap(),
        vec![BorrowRequestResponse {
            id: 1u64,
            collateral: Asset {
                info: AssetInfo::NativeToken {
                    denom: MOCK_NATIVE_TOKEN_0.to_string(),
                },
                amount,
            },
            period,
            borrowed_from: 0u64,
            borrowed_at: 0u64,
        }]
    );
}

/// ## Description
/// Test if making borrow request works properly
/// ## Test case 4
/// Input: Native token with zero amount
/// Output: Error
#[test]
fn make_borrow_request_test_case_4() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info(MOCK_ADDRESS_0, &[]);
    let msg = InstantiateMsg {};

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let amount = Uint128::from(0u128);
    let period = 17200u64;

    let info = mock_info(
        MOCK_ADDRESS_0,
        &[Coin {
            denom: MOCK_NATIVE_TOKEN_0.to_string(),
            amount,
        }],
    );
    let msg = ExecuteMsg::MakeBorrowRequestNativeToken { period };

    let res = execute(deps.as_mut(), mock_env(), info, msg);

    match res {
        Err(ContractError::InvalidAmount {}) => (),
        _ => panic!("DO NOT ENTER HERE"),
    }
}

/// ## Description
/// Test if making borrow request works properly
/// ## Test case 5
/// Input: Native token with zero period
/// Output: Error
#[test]
fn make_borrow_request_test_case_5() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info(MOCK_ADDRESS_0, &[]);
    let msg = InstantiateMsg {};

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let amount = Uint128::from(1000u128);
    let period = 0u64;

    let info = mock_info(
        MOCK_ADDRESS_0,
        &[Coin {
            denom: MOCK_NATIVE_TOKEN_0.to_string(),
            amount,
        }],
    );
    let msg = ExecuteMsg::MakeBorrowRequestNativeToken { period };

    let res = execute(deps.as_mut(), mock_env(), info, msg);

    match res {
        Err(ContractError::InvalidPeriod {}) => (),
        _ => panic!("DO NOT ENTER HERE"),
    }
}

/// ## Description
/// Test if making borrow request works properly
/// ## Test case 6
/// Input: CW20 token and Native token with positive amount and positive period each
/// Output: BorrowRequest struct with the given CW20 token and native token
#[test]
fn make_borrow_request_test_case_6() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info(MOCK_ADDRESS_0, &[]);
    let msg = InstantiateMsg {};

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let amount = Uint128::from(1000u128);
    let period = 17200u64;

    let info = mock_info(MOCK_CW20_TOKEN_0, &[]);
    let msg = ExecuteMsg::MakeBorrowRequestCw20Token(Cw20ReceiveMsg {
        sender: MOCK_ADDRESS_0.to_string(),
        amount,
        msg: to_binary(&Cw20HookMsg::MakeBorrowRequest { period }).unwrap(),
    });

    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let info = mock_info(
        MOCK_ADDRESS_0,
        &[Coin {
            denom: MOCK_NATIVE_TOKEN_0.to_string(),
            amount,
        }],
    );
    let msg = ExecuteMsg::MakeBorrowRequestNativeToken { period };

    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    assert_eq!(
        from_binary::<Vec<BorrowRequestResponse>>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::BorrowRequestByAddr {
                    borrower: MOCK_ADDRESS_0.to_string(),
                },
            )
            .unwrap(),
        )
        .unwrap(),
        vec![
            BorrowRequestResponse {
                id: 1u64,
                collateral: Asset {
                    info: AssetInfo::Token {
                        contract_addr: Addr::unchecked(MOCK_CW20_TOKEN_0.to_string()),
                    },
                    amount,
                },
                period,
                borrowed_from: 0u64,
                borrowed_at: 0u64,
            },
            BorrowRequestResponse {
                id: 2u64,
                collateral: Asset {
                    info: AssetInfo::NativeToken {
                        denom: MOCK_NATIVE_TOKEN_0.to_string(),
                    },
                    amount,
                },
                period,
                borrowed_from: 0u64,
                borrowed_at: 0u64,
            }
        ]
    );
}
