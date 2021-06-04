import {instantiateContract, uploadContract, executeContract, queryContract} from "./helpers.mjs";
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
    // let sign_msg = { "sign_manifesto": {} };
    // let resp = await executeContract(terra, wallet, manifesto_address, sign_msg );
    // console.log(resp);

  }

  main()

//   terracli query wasm contract-store terra18vd8fpwxzck93qlwghaj6arh4p7c5n896xzem5 '{"get_count":{}}'