use pallet_contracts::chain_extension::{
    ChainExtension, Config as ContractsConfig, Environment, Ext, InitState, RetVal,
};
use pallet_fake_staking::{Config as FakeStakingConfig, Pallet as FakeStakingPallet};
use parity_scale_codec::Encode;

#[derive(Default)]
pub struct StakingExtension;

impl<Runtime: ContractsConfig + FakeStakingConfig> ChainExtension<Runtime> for StakingExtension {
    fn call<E: Ext<T = Runtime>>(
        &mut self,
        env: Environment<E, InitState>,
    ) -> pallet_contracts::chain_extension::Result<RetVal> {
        assert_eq!(env.func_id(), 41);

        let mut env = env.buf_in_buf_out();

        let account = env.read_as::<<Runtime as frame_system::Config>::AccountId>()?;
        let result = FakeStakingPallet::<Runtime>::is_validator(account);

        env.write(&result.encode(), false, None)?;

        Ok(RetVal::Converging(0))
    }
}
