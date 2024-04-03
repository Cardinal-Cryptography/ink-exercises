use ink::env::{DefaultEnvironment, Environment};
use ink::primitives::AccountId;
use ink::prelude::string::String;
use crate::VotingResult;

/// Event emitted when the voting is started.
#[ink::event]
pub struct VotingStarted {
    /// The title of the voting.
    #[ink(topic)]
    pub title: String,
    /// The deadline of the voting.
    pub deadline: <DefaultEnvironment as Environment>::BlockNumber,
}

/// Event emitted when a user votes for a voting.
#[ink::event]
pub struct VotedFor {
    /// The title of the voting.
    #[ink(topic)]
    pub title: String,
    /// The account that voted.
    pub account: AccountId,
}

/// Event emitted when a user votes against a voting.
#[ink::event]
pub struct VotedAgainst {
    /// The title of the voting.
    #[ink(topic)]
    pub title: String,
    /// The account that voted.
    pub account: AccountId,
}

/// Event emitted when the voting is done.
#[ink::event]
pub struct VotingDone {
    /// The title of the voting.
    #[ink(topic)]
    pub title: String,
    /// The result of the voting.
    pub result: VotingResult,
}
