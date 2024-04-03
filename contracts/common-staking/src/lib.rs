#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod runtime_call;

#[ink::contract]
mod common_staking {
    use ink::{
        env::{
            call::{build_call, ExecutionInput, Selector},
            DefaultEnvironment,
        },
        prelude::string::String,
        storage::Mapping,
    };

    use crate::runtime_call::{FakeStakingCall, RuntimeCall};

    #[ink(storage)]
    pub struct CommonStaking {
        already_staking: bool,
    }

    impl CommonStaking {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                already_staking: false,
            }
        }

        #[ink(message, payable, selector = 1)]
        pub fn stake(&mut self) {
            let transferred_balance = self.env().transferred_value();
            let accumulated_balance = self.env().balance();

            if accumulated_balance >= 100 && self.already_staking {
                self.env()
                    .call_runtime(&RuntimeCall::FakeStaking(FakeStakingCall::StakeMore {
                        more: transferred_balance,
                    }))
                    .expect("Failed to call FakeStaking::StakeMore");
            } else if accumulated_balance >= 100 && !self.already_staking {
                self.env()
                    .call_runtime(&RuntimeCall::FakeStaking(FakeStakingCall::Stake {
                        stake: accumulated_balance,
                    }))
                    .expect("Failed to call FakeStaking::Stake");
                self.already_staking = true;
            }
        }
    }

    impl Default for CommonStaking {
        fn default() -> Self {
            Self::new()
        }
    }
}
