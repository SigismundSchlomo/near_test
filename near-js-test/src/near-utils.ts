import {Account, ConnectConfig, keyStores} from "near-api-js";
import path from "path";
import {homedir} from "os";
import {getConfig} from "./config";
import {FinalExecutionOutcome} from "near-api-js/lib/providers";
import {FunctionCallOptions} from "near-api-js/lib/account";
import BN from "bn.js";

const ONE_NEAR_IN_YOCTO = "1000000000000000000000000";
const CREDENTIALS_DIR = ".near-credentials";
const REF_EXCHANGE_CONTRACT_ID = getConfig().ref_exchange_contract_id;
const ALLOWANCE = getConfig().allowance;

//TODO: Util function to get near in yoctoNear

const credentialsPath = path.join(homedir(), CREDENTIALS_DIR);
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);

export const getConnectionConfig = (): ConnectConfig => {
  return {
    keyStore,
    networkId: "testnet",
    nodeUrl: "https://rpc.testnet.near.org",
    headers: {},
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
  };
}

export const registerInExchange = async (account: Account): Promise<FinalExecutionOutcome> => {
  return await registerInContract(account, REF_EXCHANGE_CONTRACT_ID)
}

export const registerInContract = async (account: Account, contractId: string): Promise<FinalExecutionOutcome> => {
  const options: FunctionCallOptions = {
    contractId: contractId,
    methodName: "storage_deposit",
    args: {},
    gas: new BN(ALLOWANCE, 10),
    attachedDeposit: new BN(ONE_NEAR_IN_YOCTO, 10).mul(new BN(0.125)) // 0.125 near
  }
  return await account.functionCall(options)
}

export const registerInToken = async (account: Account, tokenId: string): Promise<FinalExecutionOutcome> => {
  return await registerInContract(account, tokenId);
}
