#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// The number of blocks a user is subscribed for when paying 1 token.
pub const BLOCKS_FOR_TOKEN: u32 = 10;

/// A simple contract that allows users to be enlisted for some period by paying a fee.
#[ink::contract]
mod enroll {
    use ink::storage::Mapping;

    use crate::BLOCKS_FOR_TOKEN;

    #[ink(storage)]
    pub struct Enroll {
        /// The mapping of subscriptions.
        ///
        /// It might contain expired subscriptions (we do not clean it).
        subscriptions: Mapping<AccountId, BlockNumber>,
    }

    impl Enroll {
        /// Creates a new `Enroll` contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                subscriptions: Mapping::new(),
            }
        }

        /// Makes the caller to be subscribed for a period proportional to the transferred value
        /// (according to the `BLOCKS_FOR_TOKEN` constant).
        #[ink(message, payable, selector = 1)]
        pub fn subscribe(&mut self) {
            let caller = self.env().caller();
            let block_number = self.env().block_number();
            let fee = self.env().transferred_value();
            if fee == 0 {
                return;
            }

            let current_subscription_end = self.subscriptions.get(caller).unwrap_or(block_number);
            let new_subscription_end = current_subscription_end
                .saturating_add(BLOCKS_FOR_TOKEN.saturating_mul(fee as u32));

            self.subscriptions.insert(caller, &new_subscription_end);
        }

        /// Returns `true` if the given account has an active subscription.
        #[ink(message, selector = 2)]
        pub fn is_active(&self, account: AccountId) -> bool {
            match self.subscriptions.get(account) {
                Some(subscription_end) => subscription_end > self.env().block_number(),
                None => false,
            }
        }
    }

    impl Default for Enroll {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use ink::env::{
            test::{default_accounts, set_caller, set_value_transferred, advance_block},
            DefaultEnvironment,
        };
        use ink::env::test::set_block_number;

        use super::*;

        /// 1. Deploy the contract.
        /// 2. Check that the subscription is not active.
        /// 3. Subscribe to the contract.
        /// 4. Check that the subscription is active.
        #[ink::test]
        fn subscribing_works() {
            let mut enroll = Enroll::new();
            let actor = default_accounts::<DefaultEnvironment>().bob;

            assert!(!enroll.is_active(actor));

            set_caller::<DefaultEnvironment>(actor);
            set_value_transferred::<DefaultEnvironment>(1);
            enroll.subscribe();

            assert!(enroll.is_active(actor));
        }

        /// 1. Deploy the contract.
        /// 2. Check that the subscription is not active.
        /// 3. Subscribe to the contract.
        /// 4. Advance the block number to the end of the subscription.
        /// 5. Check that the subscription is not active.
        #[ink::test]
        fn subscription_ends() {
            let mut enroll = Enroll::new();
            let actor = default_accounts::<DefaultEnvironment>().bob;
            let fee = 1;

            assert!(!enroll.is_active(actor));

            set_caller::<DefaultEnvironment>(actor);
            set_value_transferred::<DefaultEnvironment>(fee);
            enroll.subscribe();

            for _ in 0..BLOCKS_FOR_TOKEN * (fee as u32) {
                assert!(enroll.is_active(actor));
                advance_block::<DefaultEnvironment>();
            }
            assert!(!enroll.is_active(actor));
        }

        /// 1. Deploy the contract.
        /// 2. Subscribe to the contract.
        /// 3. Advance the block number to the end of the subscription.
        /// 4. Check that the subscription is not active.
        /// 5. Subscribe to the contract again.
        /// 6. Check that the subscription is active.
        #[ink::test]
        fn revive_subscription() {
            let mut enroll = Enroll::new();
            let actor = default_accounts::<DefaultEnvironment>().bob;
            let fee = 1;

            set_caller::<DefaultEnvironment>(actor);
            set_value_transferred::<DefaultEnvironment>(fee);
            enroll.subscribe();

            set_block_number::<DefaultEnvironment>(BLOCKS_FOR_TOKEN * (fee as u32));
            assert!(!enroll.is_active(actor));

            enroll.subscribe();

            assert!(enroll.is_active(actor));
        }
    }
}
