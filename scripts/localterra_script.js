import {instantiateContract, uploadContract, executeContract, queryContract} from "./helpers.mjs";
import {LCDClient, LocalTerra, MnemonicKey} from "@terra-money/terra.js";

async function main() {

    // LOCAL TERRA INSTANCE
    let terra = new LocalTerra();
    let wallet = terra.wallets.test1;    
    console.log("Wallet Address : " + wallet.key.accAddress )
  
    // MANIFESTO CONTRACT DEPLOYMENT
    let manifesto_id = await uploadContract(terra, wallet, '../manifesto/artifacts/manifesto_contract.wasm');
    console.log('MANIFESTO CONTRACT ID : ' + manifesto_id )

    let manifesto_init_msg = {  }
    let manifesto_address = await instantiateContract(terra, wallet, manifesto_id, manifesto_init_msg );

    console.log('MANIFESTO ADDRESS : ' + manifesto_address )

    // SIGN MANIFESTO TX
    let response = await sign_manifesto(terra, wallet, manifesto_address, "21 Mesha, 11 BML", "15:10:14 AMT"); 
  }

  async function sign_manifesto(terra, wallet, manifesto_address, martian_date, martian_time) {
    let sign_msg = { "sign_manifesto": { "martian_date":martian_date, "martian_time":martian_time } };
    let resp = await executeContract(terra, wallet, manifesto_address, sign_msg ); 
    return resp;
}

  main()

