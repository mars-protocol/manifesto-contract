#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, Binary, Addr, Env, Querier, StdError,  Deps, DepsMut, MessageInfo, Reply, ReplyOn, Response, SubMsg, WasmMsg,
    StdResult, Storage
};
use crate::msg::{SignatureResponse, SigneeResponse, CountResponse, HandleMsg, InitMsg, QueryMsg, MigrateMsg};
use crate::state::{config, config_read, State, store_signee, read_signee, Signature, create_signature, read_signature};

use protobuf::Message;

//----------------------------------------------------------------------------------------
// Entry points
//----------------------------------------------------------------------------------------

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> StdResult<Response> {

    let state = State {
        signees: i32::from(0),
    };
    config(&mut deps.storage).save(&state)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: HandleMsg) -> StdResult<Response> {
    match msg {
        HandleMsg::SignManifesto { martian_date, martian_time } => try_sign_manifesto(deps, env, martian_date, martian_time),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::IsSignee { address } => to_binary(&check_if_signee(deps, address)?),
        QueryMsg::GetSignature { signee } => to_binary(&get_signee(deps, signee)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

//----------------------------------------------------------------------------------------
// Handle functions
//----------------------------------------------------------------------------------------

/// @dev Stores signature details provided by the Signee. https://manifesto.marsprotocol.io/ : Web app to facilitate signing with Martian Date and Time
/// @param martian_date : An equivalent martian date as according to th Darian Calender
/// @param martian_time : Coordinated Martian Time is like UTC but for Mars
pub fn try_sign_manifesto(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    martian_date: String, 
    martian_time: String
) -> StdResult<Response> {

    let signee =  info.sender.as_str();

    // Verfify if Time is in the valid format
    if !is_valid_time(&martian_time) {
        return Err(StdError::generic_err(format!( "Invalid Martian Time entered")));
    }

    // Verfify if Date is in the valid format
    if !is_valid_date(&martian_date) {
        return Err(StdError::generic_err(format!( "Invalid Martian Date entered")));
    }    
    
    // Make sure the account has not already signed the Manifesto
    let res: Option<bool> = read_signee(&deps.storage).may_load( signee.as_bytes() )?;
    let is_signed = match res {
        None => false,
        Some(_res) => true,
    };
    if is_signed {
        return Err(StdError::generic_err(format!( "User has already signed the Manifesto")));
    }    

    // // Add the signee to the storage
    store_signee(&mut deps.storage).save(signee.as_bytes(), &true) ?;
    
    // STORE THE SIGNATURE
    create_signature(
        &mut deps.storage,
        signee.to_string(),
        Signature {
            signee: deps.api.addr_canonicalize(signee)?,
            martian_date: martian_date,
            martian_time: martian_time
        },
    )?;

    // Update State
    config(&mut deps.storage).update(|mut state| {
        state.signees += 1;
        Ok(state)
    })?;

    let messages= vec![];
    let attributes = vec![
        attr("action", "sign_manifesto"),
        attr("signee", signee ),
    ];

    Ok(Response { messages, attributes, data: None, events: vec![], })
}

//----------------------------------------------------------------------------------------
// Query functions
//----------------------------------------------------------------------------------------

/// @dev Returns the total number of Signee's that have signed the Manifesto
fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(CountResponse { count: state.signees })
}

/// @dev Returns True if the user has signed the manifesto
fn check_if_signee(deps: Deps, address: String) -> StdResult<SigneeResponse> {
    let res: Option<bool> = read_signee(&deps.storage).may_load( address.as_bytes() )?;
    let is_signee = match res {
        None => false,
        Some(_res) => true,
    };
    return Ok(SigneeResponse { is_signee: is_signee })
}

/// @dev Returns Signauture details of the signee
fn get_signee(deps: Deps, signee: String) -> StdResult<SignatureResponse> {
    let signature: Signature =  read_signature(&deps.storage, signee )?;
    let signee_human_addr = deps.api.addr_humanize(&signature.signee).to_string();
    return Ok(SignatureResponse {   signee: signee_human_addr ,
                                    martian_date : signature.martian_date,
                                    martian_time: signature.martian_time
                                })
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
    if  bytes.len() < 12 || bytes.len() > 24 {
        return false;
    }
    return true;
}


//----------------------------------------------------------------------------------------
// UNIT TESTS
//----------------------------------------------------------------------------------------




#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);

        let msg = InitMsg { };
        let env = mock_env("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(0, value.count);
    }

    #[test]
    fn sign_manifesto() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));
        
        // Iniitalize env
        let msg = InitMsg { };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // Sign the manifesto: Should fail with invalid date error
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::SignManifesto {martian_date:"21, 11 BML".to_string() , martian_time:"15:17:14 AMT".to_string()};
        let res_error = handle(&mut deps, env, msg);
        match res_error {
            Err(StdError::GenericErr { msg, .. }) => assert_eq!(  
                msg,
                format!( "Invalid Martian Date entered")
            ),
            _ => panic!("DO NOT ENTER HERE"),
        }

        // Sign the manifesto the first time : Should fail with invalid time error
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::SignManifesto {martian_date:"21 Mesha, 11 BML".to_string() , martian_time:"15:1:14 AMT".to_string()};
        let res_error = handle(&mut deps, env, msg);
        match res_error {
            Err(StdError::GenericErr { msg, .. }) => assert_eq!(  
                msg,
                format!( "Invalid Martian Time entered")
            ),
            _ => panic!("DO NOT ENTER HERE"),
        }

        // Sign the manifesto the first time : Should be successful
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::SignManifesto { martian_date:"20 Mesha, 11 BML".to_string() , martian_time:"14:17:14 AMT".to_string() };
        let _res = handle(&mut deps, env, msg).unwrap();

        // should increase counter by 1
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.count);

        // Sign the manifesto the first time : Should fail
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::SignManifesto {martian_date:"21 Mesha, 11 BML".to_string() , martian_time:"15:17:14 AMT".to_string()};
        let res_error = handle(&mut deps, env, msg);
        match res_error {
            Err(StdError::GenericErr { msg, .. }) => assert_eq!(  
                msg,
                format!( "User has already signed the Manifesto")
            ),
            _ => panic!("DO NOT ENTER HERE"),
        }


        let res_ = query(&deps, QueryMsg::GetCount {}).unwrap();
        let n_value: CountResponse = from_binary(&res_).unwrap();
        assert_eq!(1, n_value.count);

    }
}
