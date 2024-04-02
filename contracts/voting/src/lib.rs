#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod errors;
pub mod events;
#[cfg(test)]
mod tests;

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

#[ink::contract]
mod voting {
    use core::cmp::Ordering;

    use ink::{
        env::{
            call::{build_call, ExecutionInput, Selector},
            DefaultEnvironment,
        },
        prelude::string::String,
        storage::Mapping,
    };

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
        enroll: AccountId,
        state: State,
    }

    impl Voting {
        #[ink(constructor)]
        pub fn new(title: String, enroll: AccountId) -> Self {
            Self {
                title,
                admin: Self::env().caller(),
                enroll,
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

            self.env().emit_event(VotingDone {
                title: self.title.clone(),
                result,
            });

            Ok(result)
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

            Self::check_enroll(&self.enroll, caller)?;
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

        fn check_enroll(enroll: &AccountId, caller: &AccountId) -> Result<(), VotingError> {
            let enrolled = build_call::<DefaultEnvironment>()
                .call(*enroll)
                .call_v1()
                .exec_input(
                    ExecutionInput::new(Selector::new([0x0, 0x0, 0x0, 0x2])) // `is_active`
                        .push_arg(caller),
                )
                .returns::<bool>()
                .invoke();

            ensure!(enrolled, VotingError::NotAuthorized);
            Ok(())
        }
    }
}
