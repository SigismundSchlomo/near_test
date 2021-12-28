import {Account, connect, Contract} from "near-api-js";
import {getConnectionConfig, registerInExchange} from "./near-utils";
import {getConfig} from "./config";
import {
  depositFunds,
  getRefExchangeContract, getReturn,
  getUsersDeposit,
  getWhitelistedTokens,
  isWhitelisted
} from "./ref-exchange-utils";

const TEST_CONTRACT_ID = getConfig().test_contract_id;
const REF_EXCHANGE_CONTRACT_ID = getConfig().ref_exchange_contract_id;

const AURORA_TEST_POOL_ID = 7;


//TODO: Function to sort pools by tokens
//TODO: Function to retrieve info about swap with current settings
//TODO: Function to swap
//TODO: Function to withdraw tokens

//TODO: Functionality to handle personal whitelisted tokens ??? Research this feature

//TODO: Improve near object handling
(async function () {
  const config = getConnectionConfig();
  const near = await connect(config);

  //Account from which calls are made
  const account = await near.account(TEST_CONTRACT_ID);

  const registerResult = await registerInExchange(account);
  console.log(registerResult);

  const result = await account.viewFunction(REF_EXCHANGE_CONTRACT_ID, "get_pools", {from_index: 0, limit: 20});
  console.log(result);

  const whitelistedTokens = await getWhitelistedTokens(account);
  console.log(whitelistedTokens);

  const whitelisted = await isWhitelisted(account, "wrap.testnet");
  console.log(whitelisted);

  const userDeposit = await getUsersDeposit(account);
  console.log(userDeposit);

  try {
    const depositResult = await depositFunds(account, "wrap.testnet", "1000000000000000000000000");
    console.log(depositResult);
  } catch (error) {
    console.log(error);
  }

  try {
    const result = await getReturn(account, AURORA_TEST_POOL_ID, 'aurora.fakes.testnet', "10000000000000000", "wrap.testnet");
    console.log(result);
  } catch (error) {
    console.log(error);
  }



})();
