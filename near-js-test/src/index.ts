import {Account, keyStores, connect, WalletConnection, Contract} from "near-api-js";
import {FunctionCallOptions} from "near-api-js/lib/account";
import BN from "bn.js";
import {getConfig} from "./near";

const REF_EXCHANGE_CONTRACT_ID = "exchange.ref-dev.testnet";
const TEST_CONTRACT_ID = "dev-1639648769225-57824237775189";
const ALLOWANCE = "2500000000000";
const ONE_NEAR_IN_YOCTO = "1000000000000000000000000";

const registerInExchange = async (account: Account) => {
  const options: FunctionCallOptions = {
    contractId: REF_EXCHANGE_CONTRACT_ID,
    methodName: "storage_deposit",
    args: {},
    gas: new BN(ALLOWANCE, 10),
    attachedDeposit: new BN(ONE_NEAR_IN_YOCTO, 10).mul(new BN(0.125)) // 0.125 near
  }
  return await account.functionCall(
    options
  )
}

(async function () {
  const config = getConfig();
  const near = await connect(config);

  const account = await near.account(TEST_CONTRACT_ID);

  const registerResult = await registerInExchange(account);
  console.log(registerResult);
  // const result = await account.viewFunction(REF_EXCHANGE_CONTRACT_ID, "get_pools", {from_index: 0, limit: 20});
  // console.log(result);


})();
