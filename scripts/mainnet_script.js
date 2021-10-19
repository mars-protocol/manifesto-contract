import {instantiateContract, uploadContract} from "./helpers.mjs";
import {sign_manifesto, get_signeesCount, isSignee, get_signature} from "./manifesto_utils.js";
import {LCDClient, LocalTerra, MnemonicKey} from "@terra-money/terra.js";


async function main() {

    // TERRA TEST-NET
    let terra = new LCDClient({
      URL: 'https://fcd.terra.dev',
      chainID: 'columbus-5',
    });

    // For testing. Never commit the memo 
    let mk = new MnemonicKey({mnemonic:"",});
    let wallet = terra.wallet(mk);   
    console.log("Wallet Address : " + wallet.key.accAddress )
  

    // #################################################    
    // #########    MANIFESTO ::: DEPLOYMENT   #########
    // #################################################    
    let manifesto_id = await uploadContract(terra, wallet, '../manifesto/artifacts/manifesto_contract.wasm');
    console.log('MANIFESTO CONTRACT ID : ' + manifesto_id )
    let manifesto_init_msg = { 
      medal_addr: null,
      max_signees_limit: 1280,
     }
    let manifesto_address = await instantiateContract(terra, wallet, manifesto_id, manifesto_init_msg );
    console.log('MANIFESTO ADDRESS : ' + manifesto_address )

    // #################################################    
    // #########     MEDAL :::  DEPLOYMENT     #########
    // #################################################    
    let medal_id = await uploadContract(terra, wallet, '../manifesto/artifacts/manifesto_contract.wasm');
    console.log('MEDAL CONTRACT ID : ' + medal_id )
    let medal_init_msg = { 
      name: "MEDAL",
      symbol: "MEDAL",
      minter: manifesto_address
     };
    let medal_address = await instantiateContract(terra, wallet, medal_id, medal_init_msg );

    console.log('MEDAL ADDRESS : ' + medal_address )

    // #################################################    
    // #########     MEDAL (Redeemed) :::  DEPLOYMENT     #########
    // #################################################    

    let medal_redeemed_id = await uploadContract(terra, wallet, '../manifesto/artifacts/manifesto_contract.wasm');
    console.log('MEDAL CONTRACT ID : ' + medal_id )
    let medal_redeemed_init_msg = { 
      name: "MEDAL",
      symbol: "MEDAL",
      minter: medal_address
     };
    let medal_redeemed_address = await instantiateContract(terra, wallet, medal_redeemed_id, medal_redeemed_init_msg );

    console.log('MEDAL ADDRESS : ' + medal_redeemed_address )



    // #################################################    
    // #########     MANIFESTO :::  UPDATE MEDAL RELATED CONFIG (Address, Metadata)     #########
    // #################################################    


    // UPDATE MANIFESTO CONFIG














    // SIGN MANIFESTO TX
    let response = await sign_manifesto(terra, wallet, manifesto_address, "20 Leo, 11 BML", "24:59:59 MTC"); 

    // GET SIGNEES COUNT
    let signeesCount = await get_signeesCount(terra, manifesto_address);
    console.log( "Total Signees : " + String(signeesCount.count) )

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