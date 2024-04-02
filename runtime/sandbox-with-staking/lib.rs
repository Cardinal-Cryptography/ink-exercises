use drink::sp_externalities::Extension;
use frame_support::{
    __private::TestExternalities,
    construct_runtime, derive_impl, parameter_types,
    sp_runtime::{AccountId32, Perbill, testing::H256, traits::Convert},
    traits::{ConstBool, ConstU128, ConstU32, ConstU64, Currency, Randomness},
    weights::Weight,
};
use ink_sandbox::{AccountIdFor, BlockBuilder, RuntimeMetadataPrefixed, Sandbox};

construct_runtime!(
    pub enum RuntimeWithStaking {
        System: frame_system,
        Balances: pallet_balances,
        Timestamp: pallet_timestamp,
        Contracts: pallet_contracts,
        FakeStaking: pallet_fake_staking,
    }
);

#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for RuntimeWithStaking {
    type Block = frame_system::mocking::MockBlockU32<RuntimeWithStaking>;
    type Version = ();
    type BlockHashCount = ConstU32<250>;
    type AccountData =
        pallet_balances::AccountData<<RuntimeWithStaking as pallet_balances::Config>::Balance>;
}

impl pallet_balances::Config for RuntimeWithStaking {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Balance = u128;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<1>;
    type AccountStore = System;
    type ReserveIdentifier = [u8; 8];
    type FreezeIdentifier = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
}

// Configure pallet timestamp
impl pallet_timestamp::Config for RuntimeWithStaking {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<1>;
    type WeightInfo = ();
}

// Configure pallet contracts
pub enum SandboxRandomness {}
impl Randomness<H256, u32> for SandboxRandomness {
    fn random(_subject: &[u8]) -> (H256, u32) {
        unreachable!("No randomness")
    }
}

type BalanceOf = <Balances as Currency<AccountId32>>::Balance;
impl Convert<Weight, BalanceOf> for RuntimeWithStaking {
    fn convert(w: Weight) -> BalanceOf {
        w.ref_time().into()
    }
}

parameter_types! {
    pub SandboxSchedule: pallet_contracts::Schedule<RuntimeWithStaking> = {
        <pallet_contracts::Schedule<RuntimeWithStaking>>::default()
    };
    pub DeletionWeightLimit: Weight = Weight::zero();
    pub DefaultDepositLimit: BalanceOf = 10_000_000;
    pub CodeHashLockupDepositPercent: Perbill = Perbill::from_percent(0);
    pub MaxDelegateDependencies: u32 = 32;
}

impl pallet_contracts::Config for RuntimeWithStaking {
    type Time = Timestamp;
    type Randomness = SandboxRandomness;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallFilter = ();
    type WeightPrice = Self;
    type WeightInfo = ();
    type ChainExtension = ();
    type Schedule = SandboxSchedule;
    type CallStack = [pallet_contracts::Frame<Self>; 5];
    type DepositPerByte = ConstU128<1>;
    type DepositPerItem = ConstU128<1>;
    type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
    type MaxCodeLen = ConstU32<{ 123 * 1024 }>;
    type MaxStorageKeyLen = ConstU32<128>;
    type UnsafeUnstableInterface = ConstBool<false>;
    type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
    type Migrations = ();
    type DefaultDepositLimit = DefaultDepositLimit;
    type Debug = ();
    type CodeHashLockupDepositPercent = CodeHashLockupDepositPercent;
    type MaxDelegateDependencies = MaxDelegateDependencies;
    type RuntimeHoldReason = RuntimeHoldReason;
    type Environment = ();
    type Xcm = ();
    type ApiVersion = ();
}

impl pallet_fake_staking::Config for RuntimeWithStaking {}

// Implement `crate::Sandbox` trait

/// Default initial balance for the default account.
pub const INITIAL_BALANCE: u128 = 1_000_000_000_000_000;
pub const DEFAULT_ACCOUNT: AccountId32 = AccountId32::new([1u8; 32]);

pub struct SandboxWithStaking {
    ext: TestExternalities,
}

impl Default for SandboxWithStaking {
    fn default() -> Self {
        let ext =
            BlockBuilder::<RuntimeWithStaking>::new_ext(vec![(DEFAULT_ACCOUNT, INITIAL_BALANCE)]);
        Self { ext }
    }
}

impl Sandbox for SandboxWithStaking {
    type Runtime = RuntimeWithStaking;

    fn execute_with<T>(&mut self, execute: impl FnOnce() -> T) -> T {
        self.ext.execute_with(execute)
    }

    fn dry_run<T>(&mut self, action: impl FnOnce(&mut Self) -> T) -> T {
        // Make a backup of the backend.
        let backend_backup = self.ext.as_backend();
        // Run the action, potentially modifying storage. Ensure, that there are no pending changes
        // that would affect the reverted backend.
        let result = action(self);
        self.ext.commit_all().expect("Failed to commit changes");

        // Restore the backend.
        self.ext.backend = backend_backup;
        result
    }

    fn register_extension<E: ::core::any::Any + Extension>(&mut self, ext: E) {
        self.ext.register_extension(ext);
    }

    fn initialize_block(
        height: frame_system::pallet_prelude::BlockNumberFor<Self::Runtime>,
        parent_hash: <Self::Runtime as frame_system::Config>::Hash,
    ) {
        BlockBuilder::<Self::Runtime>::initialize_block(height, parent_hash)
    }

    fn finalize_block(
        height: frame_system::pallet_prelude::BlockNumberFor<Self::Runtime>,
    ) -> <Self::Runtime as frame_system::Config>::Hash {
        BlockBuilder::<Self::Runtime>::finalize_block(height)
    }

    fn default_actor() -> AccountIdFor<Self::Runtime> {
        DEFAULT_ACCOUNT
    }

    fn get_metadata() -> RuntimeMetadataPrefixed {
        Self::Runtime::metadata()
    }

fn convert_account_to_origin(
    account: AccountIdFor<Self::Runtime>,
    ) -> <<Self::Runtime as frame_system::Config>::RuntimeCall as frame_support::sp_runtime::traits::Dispatchable>::RuntimeOrigin{
        Some(account).into()
    }
}
