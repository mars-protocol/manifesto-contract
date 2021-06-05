use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,log,
    StdResult, Storage, Uint128, Decimal
};
use cosmwasm_bignumber::{Decimal256, Uint256};
use crate::msg::{SigneeResponse, CountResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State, store_signee, read_signee};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        signees: i32::from(0),
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
        HandleMsg::SignManifesto {} => try_sign_manifesto(deps, env),
    }
}

pub fn try_sign_manifesto<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
) -> StdResult<HandleResponse> {

    let signee =  _env.message.sender.clone();

    // Make sure the account has not already signed the Manifesto
    let res: Option<bool> = read_signee(&deps.storage).may_load( signee.to_string().as_bytes() )?;
    let is_claimed = match res {
        None => false,
        Some(_res) => true,
    };
    if is_claimed {
        return Err(StdError::generic_err(format!( "User has already signed the Manifesto")));
    }    

    // // Add the signee to the storage
    store_signee(&mut deps.storage).save(signee.to_string().as_bytes(), &true) ?;

    // Update State
    config(&mut deps.storage).update(|mut state| {
        state.signees += 1;
        Ok(state)
    })?;

    let messages= vec![];
    let log = vec![
        log("action", "sign_manifesto"),
        log("signee", signee ),
    ];

    Ok(HandleResponse { messages, log, data: None, })
}


pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::IsSignee { address } => to_binary(&check_if_signee(deps, address)?),
    }
}

fn query_count<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<CountResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(CountResponse { count: state.signees })
}


fn check_if_signee<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>, address: String) -> StdResult<SigneeResponse> {
    let res: Option<bool> = read_signee(&deps.storage).may_load( address.as_bytes() )?;
    let is_signee = match res {
        None => false,
        Some(_res) => true,
    };
    return Ok(SigneeResponse { is_signee: is_signee })
}







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

        // Sign the manifesto the first time : Should be successful
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::SignManifesto {};
        let _res = handle(&mut deps, env, msg).unwrap();

        // should increase counter by 1
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.count);

        // Sign the manifesto the first time : Should fail
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::SignManifesto {};
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
