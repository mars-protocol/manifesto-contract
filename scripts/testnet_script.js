import {instantiateContract, uploadContract} from "./helpers.mjs";
import {sign_manifesto, get_manifesto_state, isSignee, get_signature} from "./manifesto_utils.js";
import {LCDClient, LocalTerra, MnemonicKey} from "@terra-money/terra.js";


async function main() {

    // MAXIMUM NUMBER OF SIGNEES ALLOWED TO SIGN THE MANIFESTO
    const MAX_SIGNEES_ALLOWED = 1280;  

    // TERRA TEST-NET
    let terra = new LCDClient({
      URL: "https://tequila-lcd.terra.dev",
      chainID: "tequila-0004",
    });

    // For testing. Never commit the memo 
    let mk = new MnemonicKey({mnemonic:process.env.TEST_WALLET,});
    let wallet = terra.wallet(mk);   
    console.log("Wallet Address : " + wallet.key.accAddress )
  
    // MANIFESTO CONTRACT DEPLOYMENT
    // let manifesto_id = await uploadContract(terra, wallet, '../manifesto/artifacts/manifesto_contract.wasm');
    // console.log('MANIFESTO CONTRACT ID : ' + manifesto_id )

    // // MANIFESTO CONTRACT ::: INSTANTIATION    
    // let manifesto_init_msg = { max_signees_allowed: MAX_SIGNEES_ALLOWED }
    // let init_memo = "INSTANTIATE MARS MANIFESTO"    
    // let manifesto_address = await instantiateContract(terra, wallet, manifesto_id, manifesto_init_msg, init_memo );
    let manifesto_address = "terra19zcmn8lgqvffj65n0qltzlp9ykqwkzlf6wmn48"
    console.log('MANIFESTO ADDRESS : ' + manifesto_address )

    // SIGN MANIFESTO TX
    let response = await sign_manifesto(terra, wallet, manifesto_address, "20 Leo, 11 BML", "24:59:59 MTC"); 

    // GET SIGNEES COUNT
    let manifesto_state = await get_manifesto_state(terra, manifesto_address);
    console.log( "Signees Count : " + String(manifesto_state.signees_count) )
    console.log( "Max Signees Allowed : " + String(manifesto_state.max_signees_allowed) )

    // CHECK IF THE ADDRESS IS THE SIGNEE
    let isSignee_ = await isSignee(terra, manifesto_address, wallet.key.accAddress);
    console.log("IS SIGNEE : " + isSignee_.is_signee)

    // GET SIGNATURE
    let signature_ = await get_signature(terra, manifesto_address,wallet.key.accAddress);
    if (signature_ && signature_.signee == wallet.key.accAddress) {
      console.log("SIGNATURE DETAILS : \n Signee : " + signature_.signee + " \n Martian Date : " + signature_.martian_date + " \n Martian Time : " + signature_.martian_time )
    } 
  }


  main()