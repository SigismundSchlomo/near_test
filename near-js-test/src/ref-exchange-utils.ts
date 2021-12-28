import {FunctionCallOptions} from "near-api-js/lib/account";
import {getConfig} from "./config";
import {Account, Contract} from "near-api-js";

const REF_EXCHANGE_CONTRACT_ID = getConfig().ref_exchange_contract_id;

export const getRefExchangeContract = (account: Account) => {
  return new Contract(
    account,
    REF_EXCHANGE_CONTRACT_ID,
    {
      viewMethods: ["get_whitelisted_tokens"],
      changeMethods: []
    }
  )
}