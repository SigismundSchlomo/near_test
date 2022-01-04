//TODO: Add functionality to take configuration from environment

// Holds a lot of constants for now

export const getConfig = () => {
  // return {
  //   ref_exchange_contract_id: "ref-finance-101.testnet",
  //   ref_farming_contract_id: "v2.ref-farming.testnet",
  //   test_contract_id: "dev-1639648769225-57824237775189",
  //   allowance: "300000000000000"
  // }
  return {
    ref_exchange_contract_id: "v2.ref-finance.near",
    ref_farming_contract_id: "v2.ref-farming.near",
    test_contract_id: "sigmund.near",
    allowance: "300000000000000",
  }
}