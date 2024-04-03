#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod runtime_call;

pub const THRESHOLD: u128 = 100;

/// Common staking contract allows users to deposit funds that will be accumulated and then staked
/// together in the `FakeStaking` pallet.
#[ink::contract]
mod common_staking {
    use crate::runtime_call::{FakeStakingCall, RuntimeCall};
    use crate::THRESHOLD;

    #[ink(storage)]
    pub struct CommonStaking {
        already_staking: bool,
    }

    impl CommonStaking {
        /// Creates a new `CommonStaking` contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                already_staking: false,
            }
        }

        /// Deposits the transferred balance into the contract.
        /// 
        /// 1. If the accumulated balance is now greater than or equal to `THRESHOLD`, then the
        /// contract will call the `FakeStaking` pallet to stake the accumulated balance.
        /// 2. If the accumulated balance is greater than or equal to `THRESHOLD` and the contract
        /// is already staking, then the contract will call the `FakeStaking` pallet to increase the
        /// stake by the transferred balance.
        /// 3. If the accumulated balance is less than `THRESHOLD`, then the contract will just
        /// accumulate the transferred balance.
        #[ink(message, payable, selector = 1)]
        pub fn stake(&mut self) {
            // todo: implement
        }
    }

    impl Default for CommonStaking {
        fn default() -> Self {
            Self::new()
        }
    }
}
