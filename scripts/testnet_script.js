import {instantiateContract, uploadContract, executeContract, queryContract, migrate} from "./helpers.mjs";
import {LCDClient, LocalTerra, MnemonicKey} from "@terra-money/terra.js";

async function main() {

    // TERRA TEST-NET
    let terra = new LCDClient({
      URL: "https://tequila-lcd.terra.dev",
      chainID: "tequila-0004",
    });

    // For testing. Never commit the memo 
    let mk = new MnemonicKey({mnemonic:"clutch panel dizzy track file recycle judge east cement angle vivid athlete person absorb horror cradle march spend glove arena illegal doll empower property",});
    let wallet = terra.wallet(mk);   
    console.log("Wallet Address : " + wallet.key.accAddress )
  
    // MANIFESTO CONTRACT DEPLOYMENT
    let manifesto_id = await uploadContract(terra, wallet, '../manifesto/artifacts/manifesto_contract.wasm');
    console.log('MANIFESTO CONTRACT ID : ' + manifesto_id )

    let manifesto_init_msg = { }
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