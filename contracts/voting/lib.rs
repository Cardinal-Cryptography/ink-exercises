#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod errors;
pub mod events;

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

#[ink::contract]
mod voting {
    use core::cmp::Ordering;
    use ink::{prelude::string::String, storage::Mapping};

    use crate::{errors::*, events::*, VotingResult};

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
    pub struct Voting {
        title: String,
        admin: AccountId,
        state: State,
    }

    impl Voting {
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
            self.state = State::Active {
                voters: Mapping::new(),
                votes_for: 0,
                votes_against: 0,
                deadline,
            };

            self.env().emit_event(VotingStarted {
                title: self.title.clone(),
                deadline,
            });

            Ok(())
        }

        #[ink(message, selector = 2)]
        pub fn vote_for(&mut self) -> Result<(), VotingError> {
            let caller = self.env().caller();
            self.vote(&caller, true)?;
            self.env().emit_event(VotedFor {
                title: self.title.clone(),
                account: caller,
            });

            Ok(())
        }

        #[ink(message, selector = 3)]
        pub fn vote_against(&mut self) -> Result<(), VotingError> {
            let caller = self.env().caller();
            self.vote(&caller, false)?;
            self.env().emit_event(VotedAgainst {
                title: self.title.clone(),
                account: caller,
            });

            Ok(())
        }

        #[ink(message, selector = 4)]
        pub fn end_voting(&mut self) -> Result<(), VotingError> {
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

            self.env().emit_event(VotingDone {
                title: self.title.clone(),
                result,
            });

            self.env().terminate_contract(self.admin);
        }

        fn vote(&mut self, caller: &AccountId, vote: bool) -> Result<(), VotingError> {
            let now = self.env().block_number();
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

            if voters.insert(*caller, &()).is_some() {
                return Err(VotingError::AlreadyVoted);
            }

            if vote {
                *votes_for = votes_for.saturating_add(1);
            } else {
                *votes_against = votes_against.saturating_add(1);
            }

            Ok(())
        }
    }
}
