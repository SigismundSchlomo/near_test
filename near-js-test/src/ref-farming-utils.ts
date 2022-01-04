import {Account, Contract} from "near-api-js";
import {getConfig} from "./config";

const REF_FARMING_CONTRACT_ID = getConfig().ref_farming_contract_id;

export const listRewards = async (account: Account): Promise<unknown> => {
  return await account.viewFunction(
    REF_FARMING_CONTRACT_ID,
    "list_rewards",
    {
      account_id: account.accountId
    }
  )
}

export const listUserSeeds = async (account: Account): Promise<unknown> => {
  return await account.viewFunction(
    REF_FARMING_CONTRACT_ID,
    "list_user_seeds",
    {
      "account_id": account.accountId
    }
  )
}

