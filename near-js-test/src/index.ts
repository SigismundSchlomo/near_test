import {Account, connect, Contract} from "near-api-js";
import {getConnectionConfig, registerInExchange} from "./near-utils";
import {getConfig} from "./config";
import {getRefExchangeContract} from "./ref-exchange-utils";

const TEST_CONTRACT_ID = getConfig().test_contract_id;
const REF_EXCHANGE_CONTRACT_ID = getConfig().ref_exchange_contract_id;

const AURORA_TEST_POOL_ID = 7;


//TODO: Function to sort pools by tokens
//TODO: Function to retrieve info about swap with current settings
//TODO: Function to deposit tokens to ref finance contract
//TODO: Function to swap
//TODO: Function to withdraw tokens
//TODO: Load list of whitelisted tokens

//TODO: Functionality to handle personal whitelisted tokens ???



// const depositExchangeToken = () => {
//
// }

//
// const swap = (account: Account) => {
//   const callOptions: FunctionCallOptions = {
//     contractId: REF_EXCHANGE_CONTRACT_ID,
//     methodName: "swap",
//     args: {}, //TODO: Add args
//
//   }
// }

//TODO: Improve near object handling
(async function () {
  const config = getConnectionConfig();
  const near = await connect(config);

  const account = await near.account(TEST_CONTRACT_ID);

  const registerResult = await registerInExchange(account);
  console.log(registerResult);

  const result = await account.viewFunction(REF_EXCHANGE_CONTRACT_ID, "get_pools", {from_index: 0, limit: 20});
  console.log(result);

  const refContract = getRefExchangeContract(account);
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  const whitelistedTokens = await refContract.get_whitelisted_tokens();
  console.log(whitelistedTokens);

})();
