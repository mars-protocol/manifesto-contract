import { executeContract, queryContract} from "./helpers.mjs";


  export async function sign_manifesto(terra, wallet, manifesto_address, martian_date, martian_time) {
    let sign_msg = { "sign_manifesto": { "martian_date":martian_date, "martian_time":martian_time } };
    try {
      let resp = await executeContract(terra, wallet, manifesto_address, sign_msg ); 
      console.log("Manifesto successfully signed by " + wallet.key.addr);
    }
    catch (error) {
      console.log(error["raw_log"]);
    }
  }

  export async function get_manifesto_state(terra, manifesto_address) {
    let query = { "config": {} };
    let resp = await queryContract(terra, manifesto_address, query ); 
    return resp;
  }

  export async function isSignee(terra, manifesto_address, userAddress) {
    let query = { "is_signee": { "address":userAddress} };
    let resp = await queryContract(terra, manifesto_address, query ); 
    return resp;
  }

  export async function get_signature(terra, manifesto_address, userAddress) {
    let query = { "get_signature": {"signee":userAddress} };
    let resp = await queryContract(terra, manifesto_address, query ); 
    return resp;
  }