import {instantiateContract, uploadContract, executeContract, queryContract} from "./helpers.mjs";
import {LCDClient, LocalTerra, MnemonicKey} from "@terra-money/terra.js";

async function main() {

    // LOCAL TERRA INSTANCE
    let terra = new LocalTerra();
    let wallet = terra.wallets.test1;    
    console.log("Wallet Address : " + wallet.key.accAddress )
  
    // MARS MEDAL TOKEN DEPLOYMENT
    let manifesto_id = await uploadContract(terra, wallet, '../manifesto/artifacts/manifesto_contract.wasm');
    console.log('MANIFESTO CONTRACT ID : ' + manifesto_id )

    let manifesto_init_msg = {
        "count": 0,
    }
    let manifesto_address = await instantiateContract(terra, wallet, manifesto_id, manifesto_init_msg );

    console.log('MANIFESTO ADDRESS : ' + manifesto_address )
    // let curBalance = await queryContract(terra, manifesto_address, {"balance": {"address": wallet.key.accAddress}} );
    // console.log(curBalance);



    // SIGN MANIFESTO TX
    let sign_msg = { "sign_manifesto": {} };
    let resp = await executeContract(terra, wallet, manifesto_address, sign_msg );
    console.log(resp);

    // let newBalance = await queryContract(terra, mars_medal_token_address, {"balance": {"address": wallet.key.accAddress}} ); 
    // console.log(newBalance);


    // Burn Medal Tx
    // let burn_medal_msg = { "burn": {'amount': '1' }};
    // let resp_ = await executeContract(terra, wallet, mars_medal_token_address, burn_medal_msg );
    // // console.log(resp_);
    // let newBalance_ = await queryContract(terra, mars_medal_token_address, {"balance": {"address": wallet.key.accAddress}} );
    // console.log(newBalance_);

    
  }

  main()

//   terracli query wasm contract-store terra18vd8fpwxzck93qlwghaj6arh4p7c5n896xzem5 '{"get_count":{}}'