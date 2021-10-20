import {
    LCDClient,
    LocalTerra,
    Wallet
  } from '@terra-money/terra.js';
import { executeContract, queryContract} from "./helpers.js";

//-----------------------------------------------------

// ------ ExecuteContract :: Function signatures ------
// - TransferNft
// - SendNft
// - Approve
// - Revoke
// - ApproveAll
// - RevokeAll
// - Mint
// - RedeemMedal
// - UpdateMedalRedeemConfig
//------------------------------------------------------
//------------------------------------------------------
// ----------- Queries :: Function signatures ----------
// - OwnerOf
// - ApprovedForAll
// - NumTokens
// - NftInfo
// - AllNftInfo
// - Tokens
// - AllTokens
// - Minter
//------------------------------------------------------



export async function transfer_nft(terra: LocalTerra | LCDClient, wallet: Wallet, nft_address: string, recipient: string, token_id: string) {
  let _msg = { "transfer_nft": { "recipient":recipient, "token_id":token_id } };
  let resp = await executeContract(terra, wallet, nft_address, _msg ); 
  return resp;
}

export async function send_nft(terra: LocalTerra | LCDClient, wallet: Wallet, nft_address: string, recipient_addr: string, token_id: string, binary_msg: object) {
    let _msg = { "send_nft": { "contract":recipient_addr, "token_id":token_id, "msg": binary_msg } };
    let resp = await executeContract(terra, wallet, nft_address, _msg ); 
    return resp;
}
  
export async function approve_nft(terra: LocalTerra | LCDClient, wallet: Wallet, nft_address: string, spender: string, token_id: string, expires: object) {
    let _msg = { "approve": { "spender":spender, "token_id":token_id, "expires": expires } };
    let resp = await executeContract(terra, wallet, nft_address, _msg ); 
    return resp;
}

export async function revoke_nft(terra: LocalTerra | LCDClient, wallet: Wallet, nft_address: string, spender: string, token_id: string ) {
    let _msg = { "revoke": { "spender":spender, "token_id":token_id } };
    let resp = await executeContract(terra, wallet, nft_address, _msg ); 
    return resp;
}

export async function approve_all_nft(terra: LocalTerra | LCDClient, wallet: Wallet, nft_address: string, operator: string, expires: object ) {
    let _msg = { "approve_all": { "operator":operator, "expires":expires } };
    let resp = await executeContract(terra, wallet, nft_address, _msg ); 
    return resp;
}

export async function revoke_all_nft(terra: LocalTerra | LCDClient, wallet: Wallet, nft_address: string, operator: string ) {
    let _msg = { "revoke_all": { "operator":operator } };
    let resp = await executeContract(terra, wallet, nft_address, _msg ); 
    return resp;
}

export async function mint_nft(terra: LocalTerra | LCDClient, wallet: Wallet, nft_address: string, mint_msg: object ) {
    let _msg = mint_msg;
    let resp = await executeContract(terra, wallet, nft_address, _msg ); 
    return resp;
}

export async function redeem_medal(terra: LocalTerra | LCDClient, wallet: Wallet, nft_address: string, token_id: string ) {
    let _msg = { "redeem_medal": { "token_id":token_id } };
    let resp = await executeContract(terra, wallet, nft_address, _msg ); 
    return resp;
}

export async function update_medal_redeem_config(terra: LocalTerra | LCDClient, wallet: Wallet, nft_address: string, medal_redeem_addr: string, metadata: object ) {
    let _msg = { "update_medal_redeem_config": { "medal_redeem_addr":medal_redeem_addr, "metadata": metadata } };
    let resp = await executeContract(terra, wallet, nft_address, _msg ); 
    return resp;
}


export async function get_owner(terra: LocalTerra | LCDClient, nft_address: string, token_id: string, include_expired: any) {
    let query = { "owner_of": { "token_id":token_id, "include_expired":include_expired } };
    let resp = await queryContract(terra, nft_address, query ); 
    return resp;
}

export async function get_approved_for_all(terra: LocalTerra | LCDClient, nft_address: string, token_id: string, include_expired: any, start_after: string, limit: number) {
    let query = { "approved_for_all": { "token_id":token_id, "include_expired":include_expired, "start_after":start_after, "limit":limit } };
    let resp = await queryContract(terra, nft_address, query ); 
    return resp;
}


export async function get_num_tokens(terra: LocalTerra | LCDClient, nft_address: string) {
    let query = { "num_tokens": {  } };
    let resp = await queryContract(terra, nft_address, query ); 
    return resp;
}


export async function get_contract_info(terra: LocalTerra | LCDClient, nft_address: string) {
    let query = { "contract_info": {  } };
    let resp = await queryContract(terra, nft_address, query ); 
    return resp;
}

export async function get_nft_info(terra: LocalTerra | LCDClient, nft_address: string, token_id: string) {
    let query = { "nft_info": { "token_id":token_id  } };
    let resp = await queryContract(terra, nft_address, query ); 
    return resp;
}

export async function get_all_nft_info(terra: LocalTerra | LCDClient, nft_address: string, token_id: string, include_expired: object) {
    let query = { "all_nft_info": { "token_id":token_id, "include_expired":include_expired  } };
    let resp = await queryContract(terra, nft_address, query ); 
    return resp;
}

export async function get_tokens(terra: LocalTerra | LCDClient, nft_address: string, owner: string, start_after: string, limit: number) {
    let query = { "tokens": { "owner":owner, "start_after":start_after, "limit":limit  } };
    let resp = await queryContract(terra, nft_address, query ); 
    return resp;
}

export async function get_all_tokens(terra: LocalTerra | LCDClient, nft_address: string, start_after: string, limit: number) {
    let query = { "all_tokens": { "start_after":start_after, "limit":limit  } };
    let resp = await queryContract(terra, nft_address, query ); 
    return resp;
}

export async function get_minter(terra: LocalTerra | LCDClient, nft_address: string) {
    let query = { "minter": {  } };
    let resp = await queryContract(terra, nft_address, query ); 
    return resp;
}

