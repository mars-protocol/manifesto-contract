use cosmwasm_std::{
    attr, entry_point, to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Response, StdError, StdResult,
};

use crate::state::{Config, Signature, State, CONFIG, SIGNATURES, STATE};
use mars_community::manifesto::{
    option_string_to_addr, zero_address, ConfigResponse, ExecuteMsg, InstantiateMsg, MigrateMsg,
    QueryMsg, SignatureResponse, StateResponse,
};

//----------------------------------------------------------------------------------------
// Entry points
//----------------------------------------------------------------------------------------

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        medal_addr: option_string_to_addr(deps.api, msg.medal_addr, zero_address())?,
        max_signees_allowed: msg.max_signees_limit,
        admin: deps.api.addr_validate(&msg.admin)?,
    };

    let state = State {
        signees_count: 0u32,
    };

    CONFIG.save(deps.storage, &config)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::UpdateAdmin { new_admin } => try_update_admin(deps, info, new_admin),
        ExecuteMsg::SignManifesto {
            martian_date,
            martian_time,
        } => try_sign_manifesto(deps, info, martian_date, martian_time),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        QueryMsg::GetSignature { signee } => to_binary(&get_signature(deps, signee)?),
    }
}

// pub fn migrate<S: Storage, A: Api, Q: Querier>(
//     _deps: &mut Extern<S, A, Q>,
//     _env: Env,
//     _msg: MigrateMsg,
// ) -> StdResult<HandleResponse> {
//     return Err(StdError::generic_err(format!("Migration is not allowed")));
// }

//----------------------------------------------------------------------------------------
// Handle functions
//----------------------------------------------------------------------------------------

/// @dev Admin function to update the admin
/// @param new_admin : New admin
pub fn try_update_admin(
    deps: DepsMut,
    info: MessageInfo,
    new_admin: String,
) -> StdResult<Response> {
    let mut config = CONFIG.load(deps.storage)?;

    // Verify if called by Admin
    if info.sender != config.admin {
        return Err(StdError::generic_err("Unauthorized"));
    }

    config.admin = info.sender;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "update_admin"),
        attr("new_admin", new_admin),
    ]))
}

/// @dev Stores signature details provided by the Signee. https://manifesto.marsprotocol.io/ : Web app to facilitate signing with Martian Date and Time
/// @param martian_date : An equivalent martian date as according to th Darian Calender
/// @param martian_time : Coordinated Martian Time is like UTC but for Mars
pub fn try_sign_manifesto(
    deps: DepsMut,
    info: MessageInfo,
    martian_date: String,
    martian_time: String,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;
    let signee = info.sender.clone();

    // Verify if Time is in the valid format
    if !is_valid_time(&martian_time) {
        return Err(StdError::generic_err(format!("Invalid Martian Time")));
    }

    // Verify if Date is in the valid format
    if !is_valid_date(&martian_date) {
        return Err(StdError::generic_err(format!("Invalid Martian Date")));
    }

    // Verify if Date is in the valid format
    if state.signees_count >= config.max_signees_allowed {
        return Err(StdError::generic_err(format!("Max signee limit reached")));
    }

    // Make sure the account has not already signed the Manifesto
    let signature_ = SIGNATURES.load(deps.storage, signee.clone().to_string().as_bytes())?;
    if signature_.signee.to_string() == signee {
        return Err(StdError::generic_err(
            "User has already signed the Manifesto",
        ));
    }

    state.signees_count += 1;
    let signature_ = Signature {
        signee: signee.clone(),
        martian_date: martian_date,
        martian_time: martian_time,
    };

    // let medal_mint_msg = build_medal_mint_msg();

    STATE.save(deps.storage, &state)?;
    SIGNATURES.save(deps.storage, signee.to_string().as_bytes(), &signature_)?;

    Ok(Response::new()
        // .add_message(medal_mint_msg)
        .add_attributes(vec![
            attr("action", "sign_manifesto"),
            attr("signee", signee),
            attr("signee_count", state.signees_count.to_string()),
        ]))
}

//----------------------------------------------------------------------------------------
// Query functions
//----------------------------------------------------------------------------------------

/// @dev Returns the Medal token address and max signee's allowed to sign the Manifesto
fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;

    Ok(ConfigResponse {
        medal_addr: config.medal_addr,
        max_signees_allowed: config.max_signees_allowed,
        admin: config.admin.to_string(),
    })
}

/// @dev Returns the total number of Signee's that have signed the Manifesto
fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state = STATE.load(deps.storage)?;

    Ok(StateResponse {
        signee_count: state.signees_count,
    })
}

/// @dev Returns Signauture details of the signee
fn get_signature(deps: Deps, signee: String) -> StdResult<SignatureResponse> {
    let signature_ = SIGNATURES.load(deps.storage, signee.to_string().as_bytes())?;
    return Ok(SignatureResponse {
        signee: signature_.signee.to_string(),
        martian_date: signature_.martian_date,
        martian_time: signature_.martian_time,
    });
}

//----------------------------------------------------------------------------------------
// Helper functions
//----------------------------------------------------------------------------------------

fn is_valid_time(time: &str) -> bool {
    let bytes = time.as_bytes();
    if bytes.len() != 12 {
        return false;
    }
    return true;
}

fn is_valid_date(date: &str) -> bool {
    let bytes = date.as_bytes();
    if bytes.len() < 12 || bytes.len() > 24 {
        return false;
    }
    return true;
}

//----------------------------------------------------------------------------------------
// UNIT TESTS
//----------------------------------------------------------------------------------------

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::{mock_dependencies, mock_env};
//     use cosmwasm_std::{coins, from_binary, StdError};

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies(20, &[]);

//         let msg = InitMsg {
//             max_signees_allowed: 1280 as u32,
//         };
//         let env = mock_env("creator", &coins(1000, "earth"));

//         // we can just call .unwrap() to assert this was a success
//         let res = init(&mut deps, env, msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         // it worked, let's query the state
//         let res = query(&deps, QueryMsg::Config {}).unwrap();
//         let value: ConfigResponse = from_binary(&res).unwrap();
//         assert_eq!(0, value.signees_count);
//     }

//     #[test]
//     fn sign_manifesto() {
//         let mut deps = mock_dependencies(20, &coins(2, "token"));

//         // Iniitalize env
//         let msg = InitMsg {
//             max_signees_allowed: 2 as u32,
//         };
//         let env = mock_env("creator", &coins(2, "token"));
//         let _res = init(&mut deps, env, msg).unwrap();

//         // Sign the manifesto: Should fail with invalid date error
//         let env = mock_env("anyone", &coins(2, "token"));
//         let msg = HandleMsg::SignManifesto {
//             martian_date: "21, 11 BML".to_string(),
//             martian_time: "15:17:14 AMT".to_string(),
//         };
//         let res_error = handle(&mut deps, env, msg);
//         match res_error {
//             Err(StdError::GenericErr { msg, .. }) => {
//                 assert_eq!(msg, format!("Invalid Martian Date"))
//             }
//             _ => panic!("DO NOT ENTER HERE"),
//         }

//         // Sign the manifesto the first time : Should fail with invalid time error
//         let env = mock_env("anyone", &coins(2, "token"));
//         let msg = HandleMsg::SignManifesto {
//             martian_date: "21 Mesha, 11 BML".to_string(),
//             martian_time: "15:1:14 AMT".to_string(),
//         };
//         let res_error = handle(&mut deps, env, msg);
//         match res_error {
//             Err(StdError::GenericErr { msg, .. }) => {
//                 assert_eq!(msg, format!("Invalid Martian Time"))
//             }
//             _ => panic!("DO NOT ENTER HERE"),
//         }

//         // Sign the manifesto the first time : Should be successful
//         let env = mock_env("anyone", &coins(2, "token"));
//         let msg = HandleMsg::SignManifesto {
//             martian_date: "20 Mesha, 11 BML".to_string(),
//             martian_time: "14:17:14 AMT".to_string(),
//         };
//         let _res = handle(&mut deps, env, msg).unwrap();

//         // should increase counter by 1
//         let res = query(&deps, QueryMsg::Config {}).unwrap();
//         let value: ConfigResponse = from_binary(&res).unwrap();
//         assert_eq!(1, value.signees_count);

//         // Sign the manifesto the second time : Should fail
//         let env = mock_env("anyone", &coins(2, "token"));
//         let msg = HandleMsg::SignManifesto {
//             martian_date: "21 Mesha, 11 BML".to_string(),
//             martian_time: "15:17:14 AMT".to_string(),
//         };
//         let res_error = handle(&mut deps, env, msg);
//         match res_error {
//             Err(StdError::GenericErr { msg, .. }) => {
//                 assert_eq!(msg, format!("User has already signed the Manifesto"))
//             }
//             _ => panic!("DO NOT ENTER HERE"),
//         }

//         let res_ = query(&deps, QueryMsg::Config {}).unwrap();
//         let n_value: ConfigResponse = from_binary(&res_).unwrap();
//         assert_eq!(1, n_value.signees_count);

//         // Sign the manifesto the first time (2nd User) : Should be successful
//         let env = mock_env("secondUser", &coins(2, "token"));
//         let msg = HandleMsg::SignManifesto {
//             martian_date: "20 Mesha, 11 BML".to_string(),
//             martian_time: "14:17:14 AMT".to_string(),
//         };
//         let _res = handle(&mut deps, env, msg).unwrap();

//         // should increase counter by 1
//         let res = query(&deps, QueryMsg::Config {}).unwrap();
//         let value: ConfigResponse = from_binary(&res).unwrap();
//         assert_eq!(2, value.signees_count);

//         // Sign the manifesto by 3rd user: Should fail with max limit reached error
//         let env = mock_env("user3", &coins(2, "token"));
//         let msg = HandleMsg::SignManifesto {
//             martian_date: "21 Mesha, 11 BML".to_string(),
//             martian_time: "15:17:14 AMT".to_string(),
//         };
//         let res_error = handle(&mut deps, env, msg);
//         match res_error {
//             Err(StdError::GenericErr { msg, .. }) => {
//                 assert_eq!(msg, format!("Max signee limit reached"))
//             }
//             _ => panic!("DO NOT ENTER HERE"),
//         }
//     }
// }
