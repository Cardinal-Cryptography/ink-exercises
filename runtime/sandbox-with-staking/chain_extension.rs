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
        // Ensure that the chain extension is called with the correct function ID (41).
        assert_eq!(env.func_id(), 41);

        // We are going to use the `buf_in_buf_out` environment, which allows us to read from the
        // input buffer and write to the output buffer.
        let mut env = env.buf_in_buf_out();

        // Read the account ID from the input buffer.
        let account = env.read_as::<<Runtime as frame_system::Config>::AccountId>()?;
        // Check if the account is a validator.
        let result = FakeStakingPallet::<Runtime>::is_validator(account);

        // Write the result to the output buffer.
        env.write(&result.encode(), false, None)?;

        // Return `Converging(0)` to indicate that the chain extension executed successfully.
        Ok(RetVal::Converging(0))
    }
}
