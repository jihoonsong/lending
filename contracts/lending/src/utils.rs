use astroport::asset::{Asset, AssetInfo};
use cosmwasm_std::{Addr, Uint128};

use crate::error::ContractError;

/// ## Description
/// Returns an object of type [`Asset`]
/// Otherwise returns [`ContractError`]
/// ## Params
/// * **contract_addr** is [`Addr`]
///
/// * **amount** is [`Uint128`]
pub fn to_cw20_token(contract_addr: Addr, amount: Uint128) -> Result<Asset, ContractError> {
    if amount.is_zero() {
        return Err(ContractError::InvalidAmount {});
    }

    Ok(Asset {
        info: AssetInfo::Token { contract_addr },
        amount,
    })
}

/// ## Description
/// Returns an object of type [`Asset`]
/// Otherwise returns [`ContractError`]
/// ## Params
/// * **denom** is [`String`]
///
/// * **amount** is [`Uint128`]
pub fn to_native_token(denom: String, amount: Uint128) -> Result<Asset, ContractError> {
    if amount.is_zero() {
        return Err(ContractError::InvalidAmount {});
    }

    Ok(Asset {
        info: AssetInfo::NativeToken { denom },
        amount,
    })
}
