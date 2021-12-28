// use near_contract_standards::storage_management::{StorageBalance, StorageBalanceBounds, StorageManagement};
// use near_sdk::json_types::{U128, ValidAccountId};
// use crate::Contract;
//
// //TODO: Implement for user deposits accounting
// impl StorageManagement for Contract {
//     fn storage_deposit(&mut self, account_id: Option<ValidAccountId>, registration_only: Option<bool>) -> StorageBalance {
//         todo!()
//     }
//
//     fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance {
//         todo!()
//     }
//
//     fn storage_unregister(&mut self, force: Option<bool>) -> bool {
//         todo!()
//     }
//
//     fn storage_balance_bounds(&self) -> StorageBalanceBounds {
//         todo!()
//     }
//
//     fn storage_balance_of(&self, account_id: ValidAccountId) -> Option<StorageBalance> {
//         todo!()
//     }
// }