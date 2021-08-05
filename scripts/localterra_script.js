import {instantiateContract, uploadContract} from "./helpers.mjs";
import {sign_manifesto, get_signeesCount, isSignee, get_signature} from "./manifesto_utils.js";
import {LCDClient, LocalTerra, MnemonicKey} from "@terra-money/terra.js";
import chalk from "chalk";
import dotenv from "dotenv";


//----------------------------------------------------------------------------------------
// Parse Input Parameters
//----------------------------------------------------------------------------------------

// Parse .env
dotenv.config();
let network = "bombay";

// LCD Client
  let terra; 
  // if (network == "bombay") {
  if (!["columbus", "bombay"].includes(network)) {
    console.log(chalk.red("Error!"), "Invalid network: must be 'columbus' or 'bombay'");
    process.exit(0);
  } else {
    terra =
      network == 
      "columbus" ? new LCDClient({ URL: "https://lcd.terra.dev", chainID: "columbus-5" })
       : 
      new LCDClient({ URL: "https://bombay-lcd.terra.dev", chainID: "bombay-0008" });
  
    console.log(`\nNetwork  : ${chalk.cyan(network)}`);
  }

  // WALLET USED FOR DEPLOYING
  let deployer;
  if (!process.env.MNEMONIC) {
    console.log(chalk.red("Error!"), "MNEMONIC not provided");
    process.exit(0);
  } else {
    deployer = terra.wallet( new MnemonicKey({mnemonic: process.env.MNEMONIC}) );
    console.log(`Deployer : ${chalk.cyan(deployer.key.accAddress)}\n`);
  }
  
  // MANIFESTO CONTRACT DEPLOYMENT
  let manifesto_id = await uploadContract(terra, deployer, '../manifesto/artifacts/manifesto_contract.wasm');
  console.log(`MANIFESTO CONTRACT ID : ${chalk.cyan(manifesto_id)}`)

  let manifesto_init_msg = {}
  let manifesto_address = await instantiateContract(terra, deployer, manifesto_id, manifesto_init_msg );
  console.log(`MANIFESTO ADDRESS : ${chalk.cyan(manifesto_address)}`)

    // SIGN MANIFESTO TX
    let response = await sign_manifesto(terra, deployer, manifesto_address, "20 Leo, 11 BML", "24:59:59 MTC"); 

    // GET SIGNEES COUNT
    let signeesCount = await get_signeesCount(terra, manifesto_address);
    console.log(`Total Signees : ${chalk.cyan(String(signeesCount.count))}`)

    // CHECK IF THE ADDRESS IS THE SIGNEE
    let isSignee_ = await isSignee(terra, manifesto_address, deployer.key.accAddress);
    console.log(`IS SIGNEE : ${chalk.cyan(isSignee_.is_signee)}`)

    // GET SIGNATURE
    let signature_ = await get_signature(terra, manifesto_address,deployer.key.accAddress);
    if (signature_ && signature_.signee == deployer.key.accAddress) {
      console.log(`SIGNATURE DETAILS : \n Signee : ${chalk.cyan(signature_.signee)}  \n Martian Date : ${chalk.cyan(signature_.martian_date)}  \n Martian Time : ${chalk.cyan(signature_.martian_time)}`)
    } 
  



  // main()

