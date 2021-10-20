import {
  LCDClient,
  LocalTerra,
  Wallet
} from '@terra-money/terra.js';
import { executeContract, queryContract} from "./helpers.js";

//-----------------------------------------------------

// ------ ExecuteContract :: Function signatures ------
// - update_admin
// - update_medal_config
// - update_medal_redeem_config
// - sign_manifesto
//------------------------------------------------------
//------------------------------------------------------
// ----------- Queries :: Function signatures ----------
// - config
// - state
// - get_signature
//------------------------------------------------------



export async function manifesto_update_config(terra: LocalTerra | LCDClient, wallet: Wallet, manifesto_address: string, new_admin: string) {
  let _msg = { "update_admin": { "new_admin":new_admin } };
  let resp = await executeContract(terra, wallet, manifesto_address, _msg ); 
  return resp;
}


export async function manifesto_medal_config(terra: LocalTerra | LCDClient, wallet: Wallet, manifesto_address: string, medal_addr: string, metadata: object) {
  let _msg = { "update_medal_config": { "medal_addr":medal_addr, "metadata":metadata } };
  let resp = await executeContract(terra, wallet, manifesto_address, _msg ); 
  return resp;
}


export async function manifesto_medal_redeem_config(terra: LocalTerra | LCDClient, wallet: Wallet, manifesto_address: string, medal_redeem_addr: string, metadata: object) {
  let _msg = { "update_medal_redeem_config": { "medal_redeem_addr":medal_redeem_addr, "metadata":metadata } };
  let resp = await executeContract(terra, wallet, manifesto_address, _msg ); 
  return resp;
}


  export async function sign_manifesto(terra: LocalTerra | LCDClient, wallet: Wallet, manifesto_address: string, martian_date: string, martian_time: string) {
    let sign_msg = { "sign_manifesto": { "martian_date":martian_date, "martian_time":martian_time } };
    let resp = await executeContract(terra, wallet, manifesto_address, sign_msg ); 
    return resp;
  }






  export async function get_config(terra: LocalTerra | LCDClient, manifesto_address: string) {
    let query = { "config": {} };
    let resp = await queryContract(terra, manifesto_address, query ); 
    return resp;
  }

  export async function get_state(terra: LocalTerra | LCDClient, manifesto_address: string) {
    let query = { "state": {} };
    let resp = await queryContract(terra, manifesto_address, query ); 
    return resp;
  }

  export async function get_signature(terra: LocalTerra | LCDClient, manifesto_address: string, userAddress: string) {
    let query = { "get_signature": {"signee":userAddress} };
    let resp = await queryContract(terra, manifesto_address, query ); 
    return resp;
  }