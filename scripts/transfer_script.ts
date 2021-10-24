import {
  recover
} from "./helpers.js"
import { transfer_nft} from "./medal_utils.js";
import { LCDClient } from "@terra-money/terra.js"




async function main() {

    // TERRA TEST-NET
    let terra = new LCDClient({ URL: 'https://bombay-lcd.terra.dev', chainID: 'bombay-12'})
    let user = recover(terra, process.env.TEST_MAIN!)
    console.log(`Wallet address from seed: ${user.key.accAddress}`)

    let medal = "terra1rjq8v0vxa8hjvvfrnxfs4d39t5m7dtathcl5tl"
    let r_medal = "terra1xran6uftma9p8xzkww0pvguf2830kx73kf38we"

    let start_after = 0;
    let limit = 10;
    let medal_token_ids: Array<number> = [];
    let redeemed_medal_token_ids: Array<number> = [];

    for (let j=0;j<10; j++)  {
    let query = { "tokens": { "owner":user.key.accAddress, "start_after":start_after.toString(), "limit":limit  } };
    let res: Tokens = await terra.wasm.contractQuery( medal , query );
    start_after += limit;
    for (let i=0;i< res.tokens.length; i++) {
        let token_id = res.tokens[i];
        if (!medal_token_ids.includes(token_id)) {
        medal_token_ids.push(token_id);    
        }
    }
    if (res.tokens.length == 0) {
        break;
    }
    }
    console.log("MEDAL Tokens owned by user : ")
    console.log(medal_token_ids)
    console.log("\n")

    for (let j=0;j<10; j++)  {
    let query = { "tokens": { "owner":user.key.accAddress, "start_after":start_after.toString(), "limit":limit  } };
    let res: Tokens = await terra.wasm.contractQuery( r_medal , query );
    start_after += limit;
    for (let i=0;i< res.tokens.length; i++) {
        let token_id = res.tokens[i];
        if (!redeemed_medal_token_ids.includes(token_id)) {
        redeemed_medal_token_ids.push(token_id);    
        }
    }
    if (res.tokens.length == 0) {
        break;
    }
    }
    console.log("R-MEDAL Tokens owned by user : ")
    console.log(redeemed_medal_token_ids)
    console.log("\n")


    // let _medal_token_ids = ['10', '11', '12', '13', '14', '15','16', '17', '18', '19', '20', '21','22', '23', '24', '25', '26', '27',]
    // let _redeemed_medal_token_ids = [ '2', '3', '4', '5' ]

    // for (let i=0; i<_medal_token_ids.length; i++ ) {
        // await transfer_nft(terra, user, medal, "terra1mpr4pemaj26rdt7904tj93ctkdxd3grlx3xr5z", _medal_token_ids[i] ); 
    // }
    // for (let i=0; i<_redeemed_medal_token_ids.length; i++ ) {
        // await transfer_nft(terra, user, r_medal, "terra1mpr4pemaj26rdt7904tj93ctkdxd3grlx3xr5z", _redeemed_medal_token_ids[i] ); 
    // }

}

export interface Tokens {
    tokens: Array<number>
}
