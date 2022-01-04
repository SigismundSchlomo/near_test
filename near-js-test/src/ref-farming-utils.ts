import {Account, Contract} from "near-api-js";
import {getConfig} from "./config";
import {FunctionCall} from "near-api-js/lib/transaction";
import {FunctionCallOptions} from "near-api-js/lib/account";
import BN from "bn.js";

const REF_FARMING_CONTRACT_ID = getConfig().ref_farming_contract_id;
const DEV_CONTRACT_ID = getConfig().test_contract_id;
const ALLOWANCE = getConfig().allowance;

//TODO: Add types
export const listRewards = async (account: Account): Promise<unknown> => {
  return await account.viewFunction(
    REF_FARMING_CONTRACT_ID,
    "list_rewards",
    {
      account_id: account.accountId
    }
  )
}

//TODO: Add types
export const listUserSeeds = async (account: Account): Promise<unknown> => {
  return await account.viewFunction(
    REF_FARMING_CONTRACT_ID,
    "list_user_seeds",
    {
      "account_id": account.accountId
    }
  )
}

//TODO: Add types
export const claimReward = async (account: Account, poolId: number): Promise<unknown> => {
  const callOptions: FunctionCallOptions = {
    contractId: DEV_CONTRACT_ID,
    methodName: "claim_reward",
    args: {
      pool_id: poolId
    },
    gas: new BN(ALLOWANCE, 10),
  }
  return await account.functionCall(callOptions);
}

export const withdrawReward = async (account: Account, tokenId: string, amount: string): Promise<unknown> => {
  const callOptions: FunctionCallOptions = {
    contractId: REF_FARMING_CONTRACT_ID,
    methodName: "withdraw_reward",
    args: {
      token_id: tokenId,
      amount: amount
    },
    gas: new BN(ALLOWANCE, 10),
    attachedDeposit: new BN(1, 10),
  }
  return await account.functionCall(callOptions);
}