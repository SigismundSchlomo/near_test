import {Account, Contract} from "near-api-js";
import {getConfig} from "./config";
import {FunctionCall} from "near-api-js/lib/transaction";
import {FunctionCallOptions} from "near-api-js/lib/account";
import BN, {min} from "bn.js";
import {FinalExecutionOutcome} from "near-api-js/lib/providers";
import {getReturn, SwapAction} from "./ref-exchange-utils";
import {ALL} from "dns";

const REF_FARMING_CONTRACT_ID = getConfig().ref_farming_contract_id;
const DEV_CONTRACT_ID = getConfig().test_contract_id;
const ALLOWANCE = getConfig().allowance;

//TODO: Move external calls to separate file

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

export const addSharesToFarming = async (account: Account, poolId: number, amount: string): Promise<FinalExecutionOutcome> => {
  const callOptions: FunctionCallOptions = {
    contractId: DEV_CONTRACT_ID,
    methodName: "stake_to_farm",
    args: {
      pool_id: poolId,
      amount: amount
    },
    gas: new BN(ALLOWANCE, 10),
    attachedDeposit: new BN(1, 10) //if this is enough?
  }
  return await account.functionCall(callOptions);
}

export interface CreatePositionArgs {
  poolId: number,
  tokenIn: string,
  amountIn: string,
  tokenOut: string,
  minAmountOut?: string
}

export const createPosition = async (account: Account, args: CreatePositionArgs): Promise<FinalExecutionOutcome> => {
  const minAmount = await getReturn(account, {
    pool_id: args.poolId,
    token_in: args.tokenIn,
    amount_in: args.amountIn,
    token_out: args.tokenOut,
  });
  // args = {
  //   ...args,
  //   minAmountOut: minAmount
  // };
  const calLOptions: FunctionCallOptions = {
    contractId: DEV_CONTRACT_ID,
    methodName: "create_position",
    args: {
      pool_id: args.poolId,
      token_in: args.tokenIn,
      amount_in: args.amountIn,
      token_out: args.tokenOut,
      amount_in_pool: args.amountIn,
      min_amount_out: minAmount
    },
    gas: new BN(ALLOWANCE, 10),
    attachedDeposit: new BN(1, 10)
  }
  return await account.functionCall(calLOptions)
}