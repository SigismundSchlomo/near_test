import {connect} from "near-api-js";
import {getConnectionConfig} from "./near-utils";
import {getConfig} from "./config";
import {listRewards, listUserSeeds} from "./ref-farming-utils";
import {depositFunds, getActionWithMinAmount, getReturn, swap, SwapAction} from "./ref-exchange-utils";

const TEST_CONTRACT_ID = getConfig().test_contract_id;
const REF_EXCHANGE_CONTRACT_ID = getConfig().ref_exchange_contract_id;
const ONE_NEAR_IN_YOCTO = Math.pow(10, 24);

const TEST_POOL_ID = 0;

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
  // const account = await near.account("sigmund.near");

  // try {
  //   const swapAction = await getActionWithMinAmount(account, {
  //     pool_id: TEST_POOL_ID,
  //     token_in: "wrap.testnet",
  //     amount_in: "10000000000000000000000000",
  //     token_out: "rft.tokenfactory.testnet",
  //   });
  //   const swapResult = await swap(account, [swapAction]);
  //   console.log(swapResult);
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
