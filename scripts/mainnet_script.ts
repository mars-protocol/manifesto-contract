import chalk from "chalk";
import 'dotenv/config.js'
import {
  transferCW20Tokens,
  deployContract,
  recover,
  instantiateContract,
  uploadContract
} from "./helpers.js"
// import {sign_manifesto, get_signeesCount, isSignee, get_signature} from "./manifesto_utils.js";
import { LCDClient } from "@terra-money/terra.js"
import { join } from "path"


const ARTIFACTS_PATH = "../artifacts"


async function main() {

    // TERRA TEST-NET
    let terra = new LCDClient({ URL: 'https://bombay-lcd.terra.dev', chainID: 'bombay-12'})
    let deployer = recover(terra, process.env.TEST_MAIN!)


    console.log(`Wallet address from seed: ${deployer.key.accAddress}`)
  

    // #################################################    
    // #########    MANIFESTO ::: DEPLOYMENT   #########
    // #################################################    

    let manifesto_id = await uploadContract(terra, deployer, '../manifesto/artifacts/manifesto.wasm');
    console.log('MANIFESTO CONTRACT ID : ' + manifesto_id )
    let manifesto_init_msg = { 
      medal_addr: null,
      medal_redeem_addr: null,
      max_signees_limit: 1280,
      admin: deployer.key.accAddress
     }
    let manifesto_address = await instantiateContract(terra, deployer, manifesto_id, manifesto_init_msg );
    console.log('MANIFESTO ADDRESS : ' + manifesto_address )

    // #################################################    
    // #########     MEDAL :::  DEPLOYMENT     #########
    // #################################################    

    let medal_id = await uploadContract(terra, deployer, '../manifesto/artifacts/medal.wasm');
    console.log('MEDAL CONTRACT ID : ' + medal_id )
    let medal_init_msg = { 
      name: "MEDAL",
      symbol: "MEDAL",
      minter: manifesto_address
     };
    let medal_address = await instantiateContract(terra, deployer, medal_id, medal_init_msg );
    console.log('MEDAL ADDRESS : ' + medal_address )

    // #################################################    
    // #########     MEDAL (Redeemed) :::  DEPLOYMENT     #########
    // #################################################    

    let medal_redeemed_id = await uploadContract(terra, deployer, '../manifesto/artifacts/medal_redeemed.wasm');
    console.log('MEDAL (REDEEM) CONTRACT ID : ' + medal_id )
    let medal_redeemed_init_msg = { 
      name: "R-MEDAL",
      symbol: "RMEDAL",
      minter: medal_address
     };
    let medal_redeemed_address = await instantiateContract(terra, deployer, medal_redeemed_id, medal_redeemed_init_msg );
    console.log('MEDAL (REDEEM) ADDRESS : ' + medal_redeemed_address )

    // #################################################    
    // #########     MANIFESTO :::  UPDATE MEDAL RELATED CONFIG (Address, Metadata)     #########
    // #################################################    















    // // SIGN MANIFESTO TX
    // let response = await sign_manifesto(terra, wallet, manifesto_address, "20 Leo, 11 BML", "24:59:59 MTC"); 

    // // GET SIGNEES COUNT
    // let signeesCount = await get_signeesCount(terra, manifesto_address);
    // console.log( "Total Signees : " + String(signeesCount.count) )

    // // CHECK IF THE ADDRESS IS THE SIGNEE
    // let isSignee_ = await isSignee(terra, manifesto_address, wallet.key.accAddress);
    // console.log("IS SIGNEE : " + isSignee_.is_signee)

    // // GET SIGNATURE
    // let signature_ = await get_signature(terra, manifesto_address,wallet.key.accAddress);
    // if (signature_ && signature_.signee == wallet.key.accAddress) {
    //   console.log("SIGNATURE DETAILS : \n Signee : " + signature_.signee + " \n Martian Date : " + signature_.martian_date + " \n Martian Time : " + signature_.martian_time )
    // } 
  }


  main()