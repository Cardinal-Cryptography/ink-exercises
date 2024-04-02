#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

use frame_support::pallet_prelude::StorageVersion;

pub use pallet::*;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::DispatchResult;
    use frame_system::{ensure_signed, pallet_prelude::OriginFor};

    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[allow(deprecated)]
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn store_key(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;
            Ok(())
        }
    }
}
