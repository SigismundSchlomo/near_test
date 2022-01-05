import {FunctionCallOptions} from "near-api-js/lib/account";
import {getConfig} from "./config";
import {Account, Contract} from "near-api-js";
import BN from "bn.js";
import {FinalExecutionOutcome} from "near-api-js/lib/providers";
import {registerInContract} from "./near-utils";

const REF_EXCHANGE_CONTRACT_ID = getConfig().ref_exchange_contract_id;
const DEV_CONTRACT_ID = getConfig().test_contract_id;
const ALLOWANCE = getConfig().allowance;
//TODO: Function to check if exchange registered for token

export interface SwapAction {
  pool_id: number;
  token_in: string;
  amount_in?: string;
  token_out: string;
  min_amount_out?: string;
}

//TODO: refactor this garbage
export const getRefExchangeContract = (account: Account) => {
  return new Contract(
    account,
    REF_EXCHANGE_CONTRACT_ID,
    {
      viewMethods: ["get_whitelisted_tokens", "get_deposits", "get_return"],
      changeMethods: []
    }
  )
}

export const getReturn = async (account: Account, action: SwapAction): Promise<string> => {
  // const swapAction: SwapAction = {
  //   pool_id: poolId,
  //   token_in: tokenInId,
  //   amount_in: amountIn,
  //   token_out: tokenOutId,
  // }
  const contract = getRefExchangeContract(account);
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  return await contract.get_return(action);
}

//TODO: Add ref exchange types
export const isWhitelisted = async (account: Account, tokenId: string): Promise<boolean> => {
  const whitelistedTokens = await getWhitelistedTokens(account);
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  return whitelistedTokens.includes(tokenId);
}

//TODO: Add ref exchange types
export const getUsersDeposit = async (account: Account): Promise<unknown> => {
  const contract = getRefExchangeContract(account);
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  return await contract.get_deposits({account_id: account.accountId});
}

//TODO: Add ref exchange types
export const getWhitelistedTokens = async (account: Account): Promise<unknown> => {
  const contract = getRefExchangeContract(account)
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  return await contract.get_whitelisted_tokens();
}

export const depositFunds = async (account: Account, tokenId: string, amountInYocto: string): Promise<FinalExecutionOutcome> => {
  const callOptions: FunctionCallOptions = {
    contractId: tokenId,
    methodName: "ft_transfer_call",
    args: {receiver_id: REF_EXCHANGE_CONTRACT_ID, amount: amountInYocto, msg: ""},
    gas: new BN(ALLOWANCE, 10),
    attachedDeposit: new BN(1, 10) // 1 yoctoNEAR required by contract
  }
  return await account.functionCall(callOptions);
}

export const registerInExchange = async (account: Account): Promise<FinalExecutionOutcome> => {
  return await registerInContract(account, REF_EXCHANGE_CONTRACT_ID)
}

export const swap = async (account: Account, actions: SwapAction[]): Promise<FinalExecutionOutcome> => {
  const callOptions: FunctionCallOptions = {
    contractId: DEV_CONTRACT_ID,
    methodName: "swap",
    args: {actions: actions},
    gas: new BN(ALLOWANCE, 10),
    attachedDeposit: new BN(1, 10) // 1 yoctoNEAR
  };
  return await account.functionCall(callOptions);
}

export const getActionWithMinAmount = async (account: Account, action: SwapAction): Promise<SwapAction> => {
  const expect = await getReturn(account, action);
  return {
    ...action,
    min_amount_out: expect
  }
}