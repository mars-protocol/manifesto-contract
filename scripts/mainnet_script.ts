import chalk from "chalk";
import {
  transferCW20Tokens,
  deployContract,
  recover,
  executeContract,
  instantiateContract,
  uploadContract
} from "./helpers.js"
// import {sign_manifesto, get_signeesCount, isSignee, get_signature} from "./manifesto_utils.js";
import { LCDClient } from "@terra-money/terra.js"


const ARTIFACTS_PATH = "../artifacts"

const MEDAL_CID = "bafybeifrvvl4ckorp2eb5afvtvyls2c7w62g3n5qo2oynt4g4pp4ymvd2a"
const MEDAL_TOKEN_URI = "	bafkreihdwdcefgh4dqkjv67uzcmw7ojee6xedzdetojuzjevtenxquvyku"

const MEDAL_REDEEMED_CID = "bafybeihv3ud2qkkvbwvv6iwldwcj7aiyfsomsa4p6bjahnctvrv4lenzt4"
const MEDAL_REDEEMED_TOKEN_URI = "QmbFMke1KXqnYyBBWxB74N4c5SBnJMVAiMNRcGu6x1AwQH"


async function main() {

    // TERRA TEST-NET
    let terra = new LCDClient({ URL: 'https://bombay-lcd.terra.dev', chainID: 'bombay-12'})
    let deployer = recover(terra, process.env.TEST_MAIN!)


    console.log(`Wallet address from seed: ${deployer.key.accAddress}`)


    // #################################################    
    // #########    MANIFESTO ::: DEPLOYMENT   #########
    // #################################################    

    // let manifesto_id = await uploadContract(terra, deployer, '../artifacts/manifesto.wasm');
    // console.log('MANIFESTO CONTRACT ID : ' + manifesto_id )
    let manifesto_init_msg = { 
      medal_addr: null,
      medal_redeem_addr: null,
      max_signees_limit: 1280,
      admin: deployer.key.accAddress
     }
    // let manifesto_address = await instantiateContract(terra, deployer, manifesto_id, manifesto_init_msg );
    let manifesto_address = "terra1m0uhvmfkcwktvllwm26895lqxeg0g3pujd7dug";
    console.log('MANIFESTO ADDRESS : ' + manifesto_address )

    // #################################################    
    // #########     MEDAL :::  DEPLOYMENT     #########
    // #################################################    

    // let medal_id = await uploadContract(terra, deployer, '../artifacts/medal.wasm');
    // console.log('MEDAL CONTRACT ID : ' + medal_id )
    let medal_init_msg = { 
      name: "MEDAL",
      symbol: "MEDAL",
      minter: manifesto_address
     };
    // let medal_address = await instantiateContract(terra, deployer, medal_id, medal_init_msg );
    let medal_address = "terra1xymzjs0tssgrjqjyr37864pdny7x8qw2nng6ht";
    console.log('MEDAL ADDRESS : ' + medal_address )

    // #################################################    
    // #########     MEDAL (Redeemed) :::  DEPLOYMENT     #########
    // #################################################    

    // let medal_redeemed_id = await uploadContract(terra, deployer, '../artifacts/medal_redeemed.wasm');
    // console.log('MEDAL (REDEEM) CONTRACT ID : ' + medal_id )
    let medal_redeemed_init_msg = { 
      name: "R-MEDAL",
      symbol: "RMEDAL",
      minter: medal_address
     };
    // let medal_redeemed_address = await instantiateContract(terra, deployer, medal_redeemed_id, medal_redeemed_init_msg );
    let medal_redeemed_address = "terra14rhzv208qpxh0ewag3ghuccu34ppy2l8xlk02w";
    console.log('MEDAL (REDEEM) ADDRESS : ' + medal_redeemed_address )

    // #################################################    
    // #########     MANIFESTO :::  UPDATE MEDAL RELATED CONFIG (Address, Metadata)     #########
    // #################################################    

    console.log('\n UPDATING MANIFESTO :: ADDING MEDAL ADDR AND METADATA ')

    let update_medal_metadata_msg = { "update_medal_config": {
                                          "medal_addr": medal_address,
                                          "metadata": { 
                                            "image": "ipfs://" + MEDAL_CID,
                                            "image_data": null,
                                            "external_url": "ipfs://" + MEDAL_TOKEN_URI,
                                            "description": "A rare and coveted badge of honor for the earliest Martians. Redeemable for a physical pin to be mailed anywhere in the galaxy.",
                                            "name": "MEDAL",
                                            "attributes": null,
                                            "background_color": null,
                                            "animation_url": null,
                                            "youtube_url": null ,
                                          } 
                                        }
                                      };
                    
                    
    // await executeContract( terra, deployer, manifesto_address, update_medal_metadata_msg);
    // console.log('SUCCESSFULLY UPDATED ')


    // #################################################    
    // #########     MANIFESTO :::  UPDATE MEDAL (REDEEM) RELATED CONFIG (Address, Metadata)     #########
    // #################################################    

    console.log('\n UPDATING MANIFESTO :: ADDING MEDAL (REDEEM) ADDR AND METADATA ')
    let update_medal_redeem_metadata_msg = { "update_medal_redeem_config": {
                                          "medal_redeem_addr": medal_redeemed_address,
                                          "metadata": { 
                                            "name_prefix": "R-MEDAL",
                                            "description": "A proof of the redeemed physical Medal pin received by burning the Mars MEDAL NFT",
                                            "image": "ipfs://" + MEDAL_REDEEMED_CID,
                                            "token_uri": "ipfs://" + MEDAL_REDEEMED_TOKEN_URI
                                          } 
                                        }
                                      };
                    
                    
    await executeContract( terra, deployer, manifesto_address, update_medal_redeem_metadata_msg);
    console.log('SUCCESSFULLY UPDATED ')









    // #################################################    
    // #########     SIGN THE MANIFESTO        #########
    // #################################################    

    console.log('\n SIGNING THE MANIFESTO')
    let sign_msg = { "sign_manifesto": { "martian_date":"20 Leo, 11 BML", "martian_time":"24:59:59 MTC" } };
    let resp = await executeContract(terra, deployer, manifesto_address, sign_msg ); 
    console.log('SUCCESSFULLY SIGNED ')




    // {"minter":{}}
    // {"contract_info":{}}
    // {"num_tokens":{}}
    // {"nft_info":{ "token_id":"1408492026064021685680771516039365489"  }}
    // {"tokens":{ "owner":"terra1lnftl96z96cyqk0zd5tkwfgk4ttrdl5mf63gnp"  }}







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