import {Account, connect, Contract} from "near-api-js";
import {getConnectionConfig, registerInExchange} from "./near-utils";
import {getConfig} from "./config";
import {
  depositFunds,
  getRefExchangeContract, getReturn,
  getUsersDeposit,
  getWhitelistedTokens,
  isWhitelisted, swap, SwapAction
} from "./ref-exchange-utils";
import exp from "constants";

const TEST_CONTRACT_ID = getConfig().test_contract_id;
const REF_EXCHANGE_CONTRACT_ID = getConfig().ref_exchange_contract_id;
const ONE_NEAR_IN_YOCTO = Math.pow(10, 24);


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

  // try {
  //   const depositResult = await depositFunds(account, "wrap.testnet", "1000000000000000000000000");
  //   console.log(depositResult);
  // } catch (error) {
  //   console.log(error);
  // }

  // const deposit = await getUsersDeposit(account);
  // console.log(deposit);

  // try {
  //   const expect = await getReturn(account, expectAction);
  //   console.log(expect);
  // } catch (error) {
  //   console.log(error)
  // }
  // const swapAction: SwapAction = {
  //   ...expectAction,
  //     min_amount_out: expect
  // }
  // try {
  //   const swapResult = await swap(account, [swapAction]);
  //   console.log(swapAction);
  // } catch (error) {
  //   console.log(error);
  // }

  //TODO: Implement reinvest
  //step 1 Claim rewards from pool
  //step 2 Withdraw rewards from pool
  //step 3 Swap tokens
  //step 4 Add liquidity to pool
  //step 5 Add shares to farming


})();
