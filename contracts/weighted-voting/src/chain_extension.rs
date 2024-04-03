use ink::{
    env::{chain_extension::FromStatusCode, DefaultEnvironment, Environment},
    primitives::AccountId,
};

/// Simple chain extension that provides information about the validator status.
#[ink::chain_extension(extension = 0)]
pub trait StakingExtension {
    type ErrorCode = StakingExtensionErrorCode;

    /// Returns `true` if the given account is a validator.
    #[allow(clippy::wrong_self_convention)]
    #[ink(function = 41, handle_status = false)]
    fn is_validator(account: AccountId) -> bool;
}

/// Error codes that can be returned by the `StakingExtension`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub struct StakingExtensionErrorCode(u32);
impl FromStatusCode for StakingExtensionErrorCode {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            _ => Err(Self(status_code)),
        }
    }
}

/// Default ink environment with `StakingExtension` included.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum StakingEnvironment {}

impl Environment for StakingEnvironment {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;

    type ChainExtension = StakingExtension;
}
