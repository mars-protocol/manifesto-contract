import {instantiateContract, uploadContract} from "./helpers.mjs";
// import {sign_manifesto, get_manifesto_state, isSignee, get_signature} from "./manifesto_utils.js";
import {LCDClient, LocalTerra, MnemonicKey} from "@terra-money/terra.js";


async function main() {

    // TERRA MAIN-NET
    let terra = new LCDClient({
      URL: 'https://fcd.terra.dev',
      chainID: 'columbus-4',
    });

    // MAXIMUM NUMBER OF SIGNEES ALLOWED TO SIGN THE MANIFESTO
    const MAX_SIGNEES_ALLOWED = 1280;

    let mk = new MnemonicKey({mnemonic: process.env.MAIN_WALLET});
    let wallet = terra.wallet(mk);   
    console.log("Wallet Address : " + wallet.key.accAddress )
  
    // MANIFESTO CONTRACT ::: DEPLOYMENT
    let manifesto_id = await uploadContract(terra, wallet, '../manifesto/artifacts/manifesto_contract.wasm');
    console.log('MANIFESTO CONTRACT ID : ' + manifesto_id )

    // MANIFESTO CONTRACT ::: INSTANTIATION
    let manifesto_init_msg = { max_signees_allowed: MAX_SIGNEES_ALLOWED }
    let init_memo = "INSTANTIATE MARS MANIFESTO"
    let manifesto_address = await instantiateContract(terra, wallet, manifesto_id, manifesto_init_msg, init_memo );

    console.log('MANIFESTO ADDRESS : ' + manifesto_address )
  }


  main()