import {
    Coin,
    isTxError,
    MsgExecuteContract,
    MsgInstantiateContract,
    MsgMigrateContract,
    MsgStoreCode,
    StdFee,
    MnemonicKey
  } from '@terra-money/terra.js';
  import { readFileSync } from 'fs';

  // Related to transactions
  export const GAS_LIMIT = 30000000;
  export const GAS_AMOUNT = 4500000;

  export async function performTransaction(terra, wallet, msg) {
    console.log(wallet.key.accAddress);
    const tx = await wallet.createAndSignTx({
      msgs: [msg],
      fee: new StdFee(GAS_LIMIT, [
        new Coin("uluna", GAS_AMOUNT),
        new Coin("uusd", GAS_AMOUNT),
      ]),
    });
    const result = await terra.tx.broadcast(tx);
    // console.log(result);
    if (isTxError(result)) {
      throw new Error(
        `transaction failed. code: ${result.code}, codespace: ${result.codespace}, raw_log: ${result.raw_log}`
      );
    }
    return result
  }

  export async function uploadContract(terra, wallet, filepath) {
    const contract = readFileSync(filepath, 'base64');
    const uploadMsg = new MsgStoreCode(wallet.key.accAddress, contract);
    let result = await performTransaction(terra, wallet, uploadMsg);
    return Number(result.logs[0].eventsByType.store_code.code_id[0]) //code_id
  }
  
  export async function instantiateContract(terra, wallet, codeId, msg) {
    const instantiateMsg = new MsgInstantiateContract(wallet.key.accAddress, codeId, msg, undefined, true);
    let result = await performTransaction(terra, wallet, instantiateMsg)
    return result.logs[0].events[0].attributes[2].value //contract address
  }
  
  export async function executeContract(terra, wallet, contractAddress, msg) {
    const executeMsg = new MsgExecuteContract(wallet.key.accAddress, contractAddress, msg);
    return await performTransaction(terra, wallet, executeMsg);
  }
  
  export async function queryContract(terra, contractAddress, query) {
    return await terra.wasm.contractQuery(
      contractAddress,
      query
    )
  }

  export async function deployContract(terra, wallet, filepath, initMsg) {
    const codeId = await uploadContract(terra, wallet, filepath);
    return await instantiateContract(terra, wallet, codeId, initMsg);
  }

  export function recover(terra, mnemonic) {
    const mk = new MnemonicKey({ mnemonic: mnemonic });
    return terra.wallet(mk);
  }

  export async function migrate(terra, wallet, contractAddress, newCodeId) {
    const migrateMsg = new MsgMigrateContract(wallet.key.accAddress, contractAddress, newCodeId, {});
    return await performTransaction(terra, wallet, migrateMsg);
  }
  