use ink::env::{DefaultEnvironment, Environment};
use ink::primitives::AccountId;
use ink::prelude::string::String;
use crate::VotingResult;

#[ink::event]
pub struct VotingStarted {
    #[ink(topic)]
    pub title: String,
    pub deadline: <DefaultEnvironment as Environment>::BlockNumber,
}

#[ink::event]
pub struct VotedFor {
    #[ink(topic)]
    pub title: String,
    pub account: AccountId,
}

#[ink::event]
pub struct VotedAgainst {
    #[ink(topic)]
    pub title: String,
    pub account: AccountId,
}

#[ink::event]
pub struct VotingDone {
    #[ink(topic)]
    pub title: String,
    pub result: VotingResult,
}
