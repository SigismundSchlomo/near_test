import {connect} from "near-api-js";
import {getConnectionConfig} from "./near-utils";
import {getConfig} from "./config";
import {createPosition, CreatePositionArgs, listRewards, listUserSeeds} from "./ref-farming-utils";
import {depositFunds, getActionWithMinAmount, getReturn, swap, SwapAction} from "./ref-exchange-utils";

const TEST_CONTRACT_ID = getConfig().test_contract_id;
const REF_EXCHANGE_CONTRACT_ID = getConfig().ref_exchange_contract_id;
const ONE_NEAR_IN_YOCTO = Math.pow(10, 24);

const TEST_POOL_ID = 0;

//TODO: Create position function


(async function () {
  const config = getConnectionConfig();
  const near = await connect(config);

  //Account from which calls are made
  const account = await near.account(TEST_CONTRACT_ID);
  // const account = await near.account("sigmund.near");

  //TODO: Implement reinvest
  //step 1 Claim rewards from pool
  //step 2 Withdraw rewards from pool
  //step 3 Swap tokens
  //step 4 Add liquidity to pool
  //step 5 Add shares to farming

  try {
    const args: CreatePositionArgs = {
      poolId: 0,
      tokenIn: "wrap.testnet",
      amountIn: "10000000000000000000000000",
      tokenOut: "rft.tokenfactory.testnet",
    }
    const result = await createPosition(account, args);
    console.log(result);
  } catch (error) {
    console.log(error);
  }


})();
