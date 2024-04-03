#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]
#![allow(deprecated)] // for constant weights

use frame_support::pallet_prelude::StorageVersion;

pub use pallet::*;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

/// The minimum stake required to become a staker.
pub const THRESHOLD: u128 = 100;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::{*, DispatchResult},
        Twox64Concat,
    };
    use frame_system::{ensure_signed, pallet_prelude::OriginFor};

    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    /// Errors that can occur during the execution of the pallet.
    #[pallet::error]
    #[derive(Clone, Eq, PartialEq)]
    pub enum Error<T> {
        /// The account is already a staker. Cannot call `stake` twice.
        AlreadyStaking,
        /// The stake is not enough to become a staker.
        NotEnoughStake,
        /// The account is not a staker. Cannot call `stake_more` without calling `stake` first.
        NotStaker,
    }

    /// The set of validators.
    #[pallet::storage]
    pub type Validators<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, ()>;

    /// The stakes of the stakers.
    #[pallet::storage]
    pub type Stakers<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u128>;

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Become a validator.
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn become_validator(origin: OriginFor<T>) -> DispatchResult {
            let validator = ensure_signed(origin)?;
            Validators::<T>::insert(&validator, ());
            Ok(())
        }

        /// Stake some amount. The stake must be at least `THRESHOLD`.
        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn stake(origin: OriginFor<T>, stake: u128) -> DispatchResult {
            let staker = ensure_signed(origin)?;
            ensure!(stake >= THRESHOLD, Error::<T>::NotEnoughStake);
            Stakers::<T>::mutate(staker, |s| match s {
                Some(_) => Err(Error::<T>::AlreadyStaking),
                None => {
                    *s = Some(stake);
                    Ok(())
                }
            })?;
            Ok(())
        }

        /// Stake more amount.
        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn stake_more(origin: OriginFor<T>, more: u128) -> DispatchResult {
            let staker = ensure_signed(origin)?;
            Stakers::<T>::mutate(staker, |s| match s {
                Some(stake) => {
                    *s = Some(stake.saturating_add(more));
                    Ok(())
                }
                None => Err(Error::<T>::NotStaker),
            })?;
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Check if an account is a validator.
        pub fn is_validator(account: <T as frame_system::Config>::AccountId) -> bool {
            Validators::<T>::contains_key(&account)
        }

        /// Check the stake of an account. Returns `None` if the account is not a staker.
        pub fn stake_of(account: <T as frame_system::Config>::AccountId) -> Option<u128> {
            Stakers::<T>::get(&account)
        }
    }
}
