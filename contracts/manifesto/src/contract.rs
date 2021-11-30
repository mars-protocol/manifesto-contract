use cosmwasm_std::{
    attr, entry_point, to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, WasmMsg,
};

use crate::state::{Config, Signature, State, CONFIG, METADATA, SIGNATURES, STATE};
use mars_community::manifesto::{
    option_string_to_addr, zero_address, ConfigResponse, ExecuteMsg, InstantiateMsg,
    MedalExecuteMsg, MedalMetaData, MintMsg, QueryMsg, SignatureResponse, StateResponse,
};
use mars_community::metadata::{Metadata, Trait};

//----------------------------------------------------------------------------------------
// Entry points
//----------------------------------------------------------------------------------------

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        medal_addr: option_string_to_addr(deps.api, msg.medal_addr, zero_address())?,
        medal_redeem_addr: option_string_to_addr(deps.api, msg.medal_redeem_addr, zero_address())?,
        max_signees_allowed: msg.max_signees_limit,
        admin: deps.api.addr_validate(&msg.admin)?,
    };

    let state = State {
        signees_count: 0u64,
    };

    CONFIG.save(deps.storage, &config)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::UpdateAdmin { new_admin } => try_update_admin(deps, info, new_admin),
        ExecuteMsg::UpdateMedalConfig {
            medal_addr,
            metadata,
        } => try_update_medal_config(deps, info, medal_addr, metadata),
        ExecuteMsg::UpdateMedalRedeemConfig {
            medal_redeem_addr,
            metadata,
        } => try_update_medal_redeem_config(deps, info, medal_redeem_addr, metadata),
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

/// @dev Admin function to update MEDAL NFT Configuration
/// @param medal_addr : New MEDAL Token Address
pub fn try_update_medal_config(
    deps: DepsMut,
    info: MessageInfo,
    medal_addr: String,
    metadata: Metadata,
) -> StdResult<Response> {
    let mut config = CONFIG.load(deps.storage)?;

    // Verify if called by Admin
    if info.sender != config.admin {
        return Err(StdError::generic_err("Unauthorized"));
    }

    let medal_metadata = MedalMetaData {
        name_prefix: metadata.name.unwrap_or_else(|| "".to_string()),
        description: metadata.description.unwrap_or_else(|| "".to_string()),
        image: metadata.image.unwrap_or_else(|| "".to_string()),
        token_uri: metadata.external_url.unwrap_or_else(|| "".to_string()),
    };

    // Update & Save
    config.medal_addr = deps.api.addr_validate(&medal_addr)?;
    CONFIG.save(deps.storage, &config)?;
    METADATA.save(deps.storage, medal_addr.as_bytes(), &medal_metadata)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "update_medal_config"),
        attr("medal_addr", medal_addr),
    ]))
}

/// @dev Admin function to update MEDAL (Redeem) NFT Configuration
/// @param medal_redeem_addr : New MEDAL (Redeem) token address
pub fn try_update_medal_redeem_config(
    deps: DepsMut,
    info: MessageInfo,
    medal_redeem_addr: String,
    metadata: MedalMetaData,
) -> StdResult<Response> {
    let mut config = CONFIG.load(deps.storage)?;

    // Verify if called by Admin
    if info.sender != config.admin {
        return Err(StdError::generic_err("Unauthorized"));
    }

    let cosmos_msg = build_update_medal_redeem_addr_msg(
        config.medal_addr.to_string(),
        medal_redeem_addr.clone(),
        metadata,
    )?;

    // Update & Save
    config.medal_redeem_addr = deps.api.addr_validate(&medal_redeem_addr)?;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_message(cosmos_msg).add_attributes(vec![
        attr("action", "update_medal_redeem_config"),
        attr("medal_redeem_addr", medal_redeem_addr),
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
    let signee = info.sender;

    // Verify if Time is in the valid format
    if !is_valid_time(&martian_time) {
        return Err(StdError::generic_err("Invalid Martian Time"));
    }

    // Verify if Date is in the valid format
    if !is_valid_date(&martian_date) {
        return Err(StdError::generic_err("Invalid Martian Date"));
    }

    // Verify if signee limit is not reached yet
    if state.signees_count >= config.max_signees_allowed {
        return Err(StdError::generic_err("Max signee limit reached"));
    }

    // Make sure the account has not already signed the Manifesto
    let signature_ = SIGNATURES
        .may_load(deps.storage, signee.to_string().as_bytes())?
        .unwrap_or_default();

    if signature_.signee == signee {
        return Err(StdError::generic_err(
            "User has already signed the Manifesto",
        ));
    }

    let token_id = state.signees_count + 1;
    let medal_mint_msg = build_medal_mint_msg(
        deps.as_ref(),
        config.medal_addr.to_string(),
        token_id,
        signee.to_string(),
        martian_date.clone(),
        martian_time.clone(),
    )?;

    state.signees_count += 1;
    let signature_ = Signature {
        signee: signee.clone(),
        martian_date,
        martian_time,
    };

    STATE.save(deps.storage, &state)?;
    SIGNATURES.save(deps.storage, signee.to_string().as_bytes(), &signature_)?;

    Ok(Response::new()
        .add_message(medal_mint_msg)
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
    let signature_ = SIGNATURES.load(deps.storage, signee.as_bytes())?;
    Ok(SignatureResponse {
        signee: signature_.signee.to_string(),
        martian_date: signature_.martian_date,
        martian_time: signature_.martian_time,
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
    true
}

fn is_valid_date(date: &str) -> bool {
    let bytes = date.as_bytes();
    if bytes.len() < 12 || bytes.len() > 24 {
        return false;
    }
    true
}

/// Helper Function. Returns CosmosMsg which updates MEDAL (Redeem) address in the MEDAL Contract
pub fn build_update_medal_redeem_addr_msg(
    medal_addr: String,
    medal_redeem_addr: String,
    metadata: MedalMetaData,
) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: medal_addr,
        msg: to_binary(&MedalExecuteMsg::UpdateMedalRedeemConfig {
            medal_redeem_addr,
            metadata,
        })?,
        funds: vec![],
    }))
}

/// Helper Function. Returns CosmosMsg which updates MEDAL (Redeem) address in the MEDAL Contract
pub fn build_medal_mint_msg(
    deps: Deps,
    medal_addr: String,
    token_id: u64,
    user_addr: String,
    martian_date: String,
    martian_time: String,
) -> StdResult<CosmosMsg> {
    let metadata = METADATA.load(deps.storage, medal_addr.as_bytes())?;

    let mut attributes_vec = vec![];
    let date_attribute = Trait {
        display_type: None,
        trait_type: "martian_date".to_string(),
        value: martian_date,
    };
    attributes_vec.push(date_attribute);

    let time_attribute = Trait {
        display_type: None,
        trait_type: "martian_time".to_string(),
        value: martian_time,
    };
    attributes_vec.push(time_attribute);

    let extension_ = Metadata {
        image: Some(metadata.image.clone()),
        image_data: None,
        external_url: None,
        description: Some(metadata.description.clone()),
        name: Some(metadata.name_prefix.clone() + &" #".to_string() + &token_id.to_string()),
        attributes: Some(attributes_vec),
        background_color: None,
        animation_url: None,
        youtube_url: None,
    };

    let mint_msg = MintMsg {
        token_id: token_id.to_string(),
        owner: user_addr,
        name: metadata.name_prefix + &" #".to_string() + &token_id.to_string(),
        description: Some(metadata.description),
        image: Some(metadata.token_uri),
        extension: extension_,
    };

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: medal_addr,
        msg: to_binary(&MedalExecuteMsg::Mint(mint_msg))?,
        funds: vec![],
    }))
}
