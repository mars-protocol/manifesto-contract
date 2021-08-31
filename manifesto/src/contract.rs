use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,log,
    StdResult, Storage
};
use crate::msg::{SignatureResponse, SigneeResponse, ConfigResponse, HandleMsg, InitMsg, QueryMsg, MigrateMsg};
use crate::state::{config, config_read, State, store_signee, read_signee, Signature, create_signature, read_signature};

//----------------------------------------------------------------------------------------
// Entry points
//----------------------------------------------------------------------------------------

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        signees_count: 0 as u32,
        max_signees_allowed: _msg.max_signees_allowed
    };

    config(&mut deps.storage).save(&state)?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::SignManifesto { martian_date, martian_time } => try_sign_manifesto(deps, env, martian_date, martian_time),
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::IsSignee { address } => to_binary(&check_if_signee(deps, address)?),
        QueryMsg::GetSignature { signee } => to_binary(&get_signee(deps, signee)?),
    }
}

pub fn migrate<S: Storage, A: Api, Q: Querier>(_deps: &mut Extern<S, A, Q>, _env: Env, _msg: MigrateMsg) -> StdResult<HandleResponse> {
    return Err(StdError::generic_err(format!( "Migration is not allowed")));
}

//----------------------------------------------------------------------------------------
// Handle functions
//----------------------------------------------------------------------------------------

/// @dev Stores signature details provided by the Signee. https://manifesto.marsprotocol.io/ : Web app to facilitate signing with Martian Date and Time
/// @param martian_date : An equivalent martian date as according to th Darian Calender
/// @param martian_time : Coordinated Martian Time is like UTC but for Mars
pub fn try_sign_manifesto<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    martian_date: String, 
    martian_time: String
) -> StdResult<HandleResponse> {

    let state_ = config_read(&deps.storage).load()?;
    let signee =  _env.message.sender.clone();

    // Verify if Time is in the valid format
    if !is_valid_time(&martian_time) {
        return Err(StdError::generic_err(format!( "Invalid Martian Time")));
    }

    // Verify if Date is in the valid format
    if !is_valid_date(&martian_date) {
        return Err(StdError::generic_err(format!( "Invalid Martian Date")));
    }    
    
    // Verify if Date is in the valid format
    if state_.signees_count >= state_.max_signees_allowed {
        return Err(StdError::generic_err(format!( "Max signee limit reached")));
    }    

    // Make sure the account has not already signed the Manifesto
    let res: Option<bool> = read_signee(&deps.storage).may_load( signee.to_string().as_bytes() )?;
    let is_signed = match res {
        None => false,
        Some(_res) => true,
    };
    if is_signed {
        return Err(StdError::generic_err(format!( "User has already signed the Manifesto")));
    }    

    // Add the signee to the storage
    store_signee(&mut deps.storage).save(signee.to_string().as_bytes(), &true) ?;
    
    // STORE THE SIGNATURE
    create_signature(
        &mut deps.storage,
        signee.to_string(),
        Signature {
            signee: deps.api.canonical_address(&signee)?,
            martian_date: martian_date,
            martian_time: martian_time
        },
    )?;

    // Update State
    config(&mut deps.storage).update(|mut state| {
        state.signees_count += 1 as u32;
        Ok(state)
    })?;

    let messages= vec![];
    let log = vec![
        log("action", "sign_manifesto"),
        log("signee", signee ),
    ];

    Ok(HandleResponse { messages, log, data: None, })
}

//----------------------------------------------------------------------------------------
// Query functions
//----------------------------------------------------------------------------------------

/// @dev Returns the total number of Signee's that have signed the Manifesto
fn query_config<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<ConfigResponse> {
    let state = config_read(&deps.storage).load()?;

    Ok(ConfigResponse { 
        signees_count: state.signees_count,
        max_signees_allowed: state.max_signees_allowed
    })
}

/// @dev Returns True if the user has signed the manifesto
fn check_if_signee<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>, address: String) -> StdResult<SigneeResponse> {
    let res: Option<bool> = read_signee(&deps.storage).may_load( address.as_bytes() )?;
    let is_signee = match res {
        None => false,
        Some(_res) => true,
    };
    return Ok(SigneeResponse { is_signee: is_signee })
}

/// @dev Returns Signauture details of the signee
fn get_signee<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>, signee: String) -> StdResult<SignatureResponse> {
    let signature: Signature =  read_signature(&deps.storage, signee )?;
    let signee_human_addr = deps.api.human_address(&signature.signee)?;
    return Ok(SignatureResponse {   signee: signee_human_addr.to_string() ,
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

        let msg = InitMsg { max_signees_allowed : 1280 as u32 };
        let env = mock_env("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(&deps, QueryMsg::Config {}).unwrap();
        let value: ConfigResponse = from_binary(&res).unwrap();
        assert_eq!(0, value.signees_count);
    }

    #[test]
    fn sign_manifesto() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));
        
        // Iniitalize env
        let msg = InitMsg { max_signees_allowed : 2 as u32 };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // Sign the manifesto: Should fail with invalid date error
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::SignManifesto {martian_date:"21, 11 BML".to_string() , martian_time:"15:17:14 AMT".to_string()};
        let res_error = handle(&mut deps, env, msg);
        match res_error {
            Err(StdError::GenericErr { msg, .. }) => assert_eq!(  
                msg,
                format!( "Invalid Martian Date")
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
                format!( "Invalid Martian Time")
            ),
            _ => panic!("DO NOT ENTER HERE"),
        }

        // Sign the manifesto the first time : Should be successful
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::SignManifesto { martian_date:"20 Mesha, 11 BML".to_string() , martian_time:"14:17:14 AMT".to_string() };
        let _res = handle(&mut deps, env, msg).unwrap();

        // should increase counter by 1
        let res = query(&deps, QueryMsg::Config {}).unwrap();
        let value: ConfigResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.signees_count);

        // Sign the manifesto the second time : Should fail
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

        let res_ = query(&deps, QueryMsg::Config {}).unwrap();
        let n_value: ConfigResponse = from_binary(&res_).unwrap();
        assert_eq!(1, n_value.signees_count);

        // Sign the manifesto the first time (2nd User) : Should be successful
        let env = mock_env("secondUser", &coins(2, "token"));
        let msg = HandleMsg::SignManifesto { martian_date:"20 Mesha, 11 BML".to_string() , martian_time:"14:17:14 AMT".to_string() };
        let _res = handle(&mut deps, env, msg).unwrap();

        // should increase counter by 1
        let res = query(&deps, QueryMsg::Config {}).unwrap();
        let value: ConfigResponse = from_binary(&res).unwrap();
        assert_eq!(2, value.signees_count);

        // Sign the manifesto by 3rd user: Should fail with max limit reached error
        let env = mock_env("user3", &coins(2, "token"));
        let msg = HandleMsg::SignManifesto {martian_date:"21 Mesha, 11 BML".to_string() , martian_time:"15:17:14 AMT".to_string()};
        let res_error = handle(&mut deps, env, msg);
        match res_error {
            Err(StdError::GenericErr { msg, .. }) => assert_eq!(  
                msg,
                format!( "Max signee limit reached")
            ),
            _ => panic!("DO NOT ENTER HERE"),
        }

    }
}
