#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod chain_extension;
pub mod errors;

/// The strength of a standard vote.
pub const STANDARD_VOTE: u32 = 1;
/// The strength of a validator vote.
pub const VALIDATOR_VOTE: u32 = 5;

/// The result of a voting.
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

/// A simple voting contract. It allows users to vote for or against a proposal. Validators have
/// a stronger vote than standard users. The voting is started by the admin and ends after a
/// deadline.
#[ink::contract(env = crate::chain_extension::StakingEnvironment)]
mod weighted_voting {
    use core::cmp::Ordering;

    use ink::{prelude::string::String, storage::Mapping};

    use crate::{errors::*, VotingResult, STANDARD_VOTE, VALIDATOR_VOTE};

    /// The state of the voting contract.
    #[ink::storage_item]
    #[derive(Debug)]
    enum State {
        /// The voting is ready to start, but voting is disabled.
        Ready,
        /// The voting is active and users can vote.
        Active {
            voters: Mapping<AccountId, ()>,
            votes_for: u32,
            votes_against: u32,
            deadline: BlockNumber,
        },
        /// The voting is done, voting is disabled..
        Done,
    }

    #[ink(storage)]
    pub struct WeightedVoting {
        /// The title of the voting.
        title: String,
        /// The admin of the voting.
        admin: AccountId,
        /// The state of the voting process.
        state: State,
    }

    impl WeightedVoting {
        /// Creates a new voting contract.
        ///
        /// The `title` is the title of the voting.
        #[ink(constructor)]
        pub fn new(title: String) -> Self {
            Self {
                title,
                admin: Self::env().caller(),
                state: State::Ready,
            }
        }

        /// Starts the voting process. Can only be called by the admin. The `deadline` is the number
        /// of blocks after which the voting ends.
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

        /// Votes for the proposal.
        #[ink(message, selector = 2)]
        pub fn vote_for(&mut self) -> Result<(), VotingError> {
            self.vote(true)?;
            Ok(())
        }

        /// Votes against the proposal.
        #[ink(message, selector = 3)]
        pub fn vote_against(&mut self) -> Result<(), VotingError> {
            self.vote(false)?;
            Ok(())
        }

        /// Ends the voting process and returns the result.
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
            // todo: implement
            Ok(())
        }
    }
}
