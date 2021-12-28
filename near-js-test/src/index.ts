import {Account, connect, Contract} from "near-api-js";
import {FunctionCallOptions} from "near-api-js/lib/account";
import BN from "bn.js";
import {getConnectionConfig, registerInExchange} from "./near";
import {FinalExecutionOutcome} from "near-api-js/lib/providers";
import {getConfig} from "./config";

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

//TODO: Util function to get near in yoctoNear



(async function () {
  const config = getConnectionConfig();
  const near = await connect(config);

  const account = await near.account(TEST_CONTRACT_ID);

  const registerResult = await registerInExchange(account);
  console.log(registerResult);

  const result = await account.viewFunction(REF_EXCHANGE_CONTRACT_ID, "get_pools", {from_index: 0, limit: 20});
  console.log(result);

})();
