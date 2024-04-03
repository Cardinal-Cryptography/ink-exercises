#![cfg(test)]

use drink::{
    session::{NO_ARGS, Session},
};
use drink::sandbox_api::balance_api::BalanceAPI;

use sandbox_with_staking::SandboxWithStaking;

use crate::utils::{deploy_contract, stake_of};

pub const BOB: [u8; 32] = [2; 32];
type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[drink::contract_bundle_provider]
enum BundleProvider {}

#[drink::test(sandbox = SandboxWithStaking)]
fn user_can_deposit_their_stake(mut session: Session) -> TestResult {
    deploy_contract(&mut session)?;
    session.call::<_, ()>("stake", NO_ARGS, Some(10))??;
    Ok(())
}

#[drink::test(sandbox = SandboxWithStaking)]
fn there_is_no_actual_stake_if_the_pool_is_too_poor(mut session: Session) -> TestResult {
    let contract = deploy_contract(&mut session)?;

    session.call::<_, ()>("stake", NO_ARGS, Some(10))??;

    assert!(stake_of(&mut session, contract).is_none());
    Ok(())
}

#[drink::test(sandbox = SandboxWithStaking)]
fn there_is_an_actual_stake_if_the_pool_has_enough_money(mut session: Session) -> TestResult {
    let contract = deploy_contract(&mut session)?;

    session.call::<_, ()>("stake", NO_ARGS, Some(100))??;

    assert_eq!(stake_of(&mut session, contract), Some(100));
    Ok(())
}

#[drink::test(sandbox = SandboxWithStaking)]
fn cumulates_stake_from_many_users_and_then_stakes(mut session: Session) -> TestResult {
    let contract = deploy_contract(&mut session)?;

    session.call::<_, ()>("stake", NO_ARGS, Some(50))??;
    assert_eq!(stake_of(&mut session, contract.clone()), None);

    session.sandbox().mint_into(&BOB.into(), 1_000_000_000_000).unwrap();
    session.set_actor(BOB.into());
    session.call::<_, ()>("stake", NO_ARGS, Some(50))??;

    assert_eq!(stake_of(&mut session, contract), Some(100));
    Ok(())
}

#[drink::test(sandbox = SandboxWithStaking)]
fn stakes_more_if_new_funds_are_deposited(mut session: Session) -> TestResult {
    let contract = deploy_contract(&mut session)?;

    session.call::<_, ()>("stake", NO_ARGS, Some(500))??;
    assert_eq!(stake_of(&mut session, contract.clone()), Some(500));

    session.call::<_, ()>("stake", NO_ARGS, Some(500))??;
    assert_eq!(stake_of(&mut session, contract), Some(1000));

    Ok(())
}

mod utils {
    use drink::{
        AccountId32,
        Sandbox, session::{NO_ARGS, NO_ENDOWMENT, NO_SALT, Session},
    };

    use sandbox_with_staking::{RuntimeWithStaking, SandboxWithStaking};

    use crate::{BundleProvider, TestResult};

    pub fn deploy_contract(session: &mut Session<SandboxWithStaking>) -> TestResult<AccountId32> {
        Ok(session.deploy_bundle(
            BundleProvider::CommonStaking.bundle()?,
            "new",
            NO_ARGS,
            NO_SALT,
            NO_ENDOWMENT,
        )?)
    }

    pub fn stake_of(session: &mut Session<SandboxWithStaking>, account: AccountId32) -> Option<u128> {
        session
            .sandbox()
            .execute_with(|| pallet_fake_staking::Pallet::<RuntimeWithStaking>::stake_of(account))
    }
}
