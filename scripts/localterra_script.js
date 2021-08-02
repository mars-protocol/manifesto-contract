import {instantiateContract, uploadContract} from "./helpers.mjs";
import {sign_manifesto, get_signeesCount, isSignee, get_signature} from "./manifesto_utils.js";
import {LCDClient, LocalTerra, MnemonicKey} from "@terra-money/terra.js";
import chalk from "chalk";
import dotenv from "dotenv";
import yargs from "yargs/yargs";


//----------------------------------------------------------------------------------------
// Parse Input Parameters
//----------------------------------------------------------------------------------------

// Parse .env
dotenv.config();

// Parse options
const argv = yargs(process.argv)
  .options({
    network: {
      alias: "n",
      type: "string",
      demandOption: true,
    }
  })
  .parseSync();

  // LCD Client
  let terra; 
  if (!["columbus", "bombay"].includes(argv.network)) {
    console.log(chalk.red("Error!"), "Invalid network: must be 'columbus' or 'bombay'");
    process.exit(0);
  } else {
    terra =
      argv.network == 
      "columbus" ? new LCDClient({ URL: "https://lcd.terra.dev", chainID: "columbus-5" })
       : 
      new LCDClient({ URL: "https://bombay-lcd.terra.dev", chainID: "bombay-0008" });
  
    console.log(`\nNetwork  : ${chalk.cyan(argv.network)}`);
  }

  // WALLET FOR DEPLOYING
  let deployer;
  if (!process.env.MNEMONIC) {
    console.log(chalk.red("Error!"), "MNEMONIC not provided");
    process.exit(0);
  } else {
    deployer = terra.wallet( new MnemonicKey({mnemonic: process.env.MNEMONIC}) );
    console.log(`Deployer : ${chalk.cyan(deployer.key.accAddress)}\n`);
  }
  
    // MANIFESTO CONTRACT DEPLOYMENT
    let manifesto_id = await uploadContract(terra, wallet, '../manifesto/artifacts/manifesto_contract.wasm');
    console.log('MANIFESTO CONTRACT ID : ' + manifesto_id )

    let manifesto_init_msg = {}
    let manifesto_address = await instantiateContract(terra, wallet, manifesto_id, manifesto_init_msg );

    console.log('MANIFESTO ADDRESS : ' + manifesto_address )

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
  



  // main()

