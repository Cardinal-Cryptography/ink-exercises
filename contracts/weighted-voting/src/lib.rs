#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod chain_extension;
pub mod errors;

pub const STANDARD_VOTE: u32 = 1;
pub const VALIDATOR_VOTE: u32 = 5;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum VotingResult {
    For,
    Against,
    Draw,
}

/// Evaluate `condition` and if not true return `Err(fallback)`.
///
/// Used as `ensure!(condition, fallback)`.
#[macro_export]
macro_rules! ensure {
    ( $condition:expr, $fallback:expr $(,)? ) => {{
        if !$condition {
            return Err($fallback.into());
        }
    }};
}

#[ink::contract(env = crate::chain_extension::StakingEnvironment)]
mod weighted_voting {
    use core::cmp::Ordering;

    use ink::{prelude::string::String, storage::Mapping};

    use crate::{errors::*, VotingResult, STANDARD_VOTE, VALIDATOR_VOTE};

    #[ink::storage_item]
    #[derive(Debug)]
    enum State {
        Ready,
        Active {
            voters: Mapping<AccountId, ()>,
            votes_for: u32,
            votes_against: u32,
            deadline: BlockNumber,
        },
        Done,
    }

    #[ink(storage)]
    pub struct WeightedVoting {
        title: String,
        admin: AccountId,
        state: State,
    }

    impl WeightedVoting {
        #[ink(constructor)]
        pub fn new(title: String) -> Self {
            Self {
                title,
                admin: Self::env().caller(),
                state: State::Ready,
            }
        }

        #[ink(message, selector = 1)]
        pub fn start_voting(&mut self, deadline: BlockNumber) -> Result<(), VotingError> {
            ensure!(self.env().caller() == self.admin, VotingError::NotAdmin);
            let deadline = self.env().block_number().saturating_add(deadline);
            self.state = State::Active {
                voters: Mapping::new(),
                votes_for: 0,
                votes_against: 0,
                deadline,
            };

            Ok(())
        }

        #[ink(message, selector = 2)]
        pub fn vote_for(&mut self) -> Result<(), VotingError> {
            self.vote(true)?;
            Ok(())
        }

        #[ink(message, selector = 3)]
        pub fn vote_against(&mut self) -> Result<(), VotingError> {
            self.vote(false)?;
            Ok(())
        }

        #[ink(message, selector = 4)]
        pub fn end_voting(&mut self) -> Result<VotingResult, VotingError> {
            ensure!(self.env().caller() == self.admin, VotingError::NotAdmin);
            let now = self.env().block_number();

            let State::Active {
                votes_for,
                votes_against,
                deadline,
                ..
            } = self.state
            else {
                return Err(VotingError::CannotEndVoting);
            };
            ensure!(now >= deadline, VotingError::CannotEndVoting);

            let result = match votes_against.cmp(&votes_for) {
                Ordering::Less => VotingResult::For,
                Ordering::Equal => VotingResult::Draw,
                Ordering::Greater => VotingResult::Against,
            };

            Ok(result)
        }

        fn vote(&mut self, vote: bool) -> Result<(), VotingError> {
            let now = self.env().block_number();
            let caller = self.env().caller();

            let strength = match self.env().extension().is_validator(caller) {
                true => VALIDATOR_VOTE,
                false => STANDARD_VOTE,
            };

            let State::Active {
                ref mut voters,
                ref mut votes_for,
                ref mut votes_against,
                deadline,
            } = self.state
            else {
                return Err(VotingError::VotingNotActive);
            };
            ensure!(now < deadline, VotingError::VotingNotActive);

            if voters.insert(caller, &()).is_some() {
                return Err(VotingError::AlreadyVoted);
            }

            if vote {
                *votes_for = votes_for.saturating_add(strength);
            } else {
                *votes_against = votes_against.saturating_add(strength);
            }

            Ok(())
        }
    }
}
