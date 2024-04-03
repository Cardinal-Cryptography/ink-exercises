#![cfg(test)]

use drink::{
    Sandbox,
    sandbox_api::{balance_api::BalanceAPI, system_api::SystemAPI},
    session::{NO_ARGS, NO_ENDOWMENT, Session},
};

use sandbox_with_staking::SandboxWithStaking;
use weighted_voting::{errors::VotingError, VotingResult};

use crate::utils::{become_validator, deploy_contract};

pub const BOB: [u8; 32] = [2; 32];
pub const CHARLIE: [u8; 32] = [3; 32];
type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[drink::contract_bundle_provider]
enum BundleProvider {}

#[drink::test(sandbox = SandboxWithStaking)]
fn standard_voting_works(mut session: Session) -> TestResult {
    deploy_contract(&mut session)?;

    let mut session = session
        .call_and("start_voting", &["1"], NO_ENDOWMENT)?
        .call_and("vote_for", NO_ARGS, NO_ENDOWMENT)?;

    session.sandbox().build_block();

    let result = session
        .call::<_, Result<VotingResult, VotingError>>("end_voting", NO_ARGS, NO_ENDOWMENT)??
        .expect("end_voting failed");

    assert_eq!(result, VotingResult::For);

    Ok(())
}

#[drink::test(sandbox = SandboxWithStaking)]
fn standard_voting_works_multiple_actors(mut session: Session) -> TestResult {
    deploy_contract(&mut session)?;

    session
        .sandbox()
        .mint_into(&BOB.into(), 1_000_000_000_000)
        .unwrap();
    session
        .sandbox()
        .mint_into(&CHARLIE.into(), 1_000_000_000_000)
        .unwrap();

    let mut session = session
        .call_and("start_voting", &["1"], NO_ENDOWMENT)?
        .call_and("vote_for", NO_ARGS, NO_ENDOWMENT)?
        .with_actor(BOB.into())
        .call_and("vote_against", NO_ARGS, NO_ENDOWMENT)?
        .with_actor(CHARLIE.into())
        .call_and("vote_against", NO_ARGS, NO_ENDOWMENT)?;

    session.sandbox().build_block();

    let result = session
        .with_actor(SandboxWithStaking::default_actor())
        .call::<_, Result<VotingResult, VotingError>>("end_voting", NO_ARGS, NO_ENDOWMENT)??
        .expect("end_voting failed");

    assert_eq!(result, VotingResult::Against);

    Ok(())
}

#[drink::test(sandbox = SandboxWithStaking)]
fn validator_overvotes_others(mut session: Session) -> TestResult {
    deploy_contract(&mut session)?;

    become_validator(&mut session, SandboxWithStaking::default_actor().into());

    session
        .sandbox()
        .mint_into(&BOB.into(), 1_000_000_000_000)
        .unwrap();
    session
        .sandbox()
        .mint_into(&CHARLIE.into(), 1_000_000_000_000)
        .unwrap();

    let mut session = session
        .call_and("start_voting", &["1"], NO_ENDOWMENT)?
        .call_and("vote_for", NO_ARGS, NO_ENDOWMENT)?
        .with_actor(BOB.into())
        .call_and("vote_against", NO_ARGS, NO_ENDOWMENT)?
        .with_actor(CHARLIE.into())
        .call_and("vote_against", NO_ARGS, NO_ENDOWMENT)?;

    session.sandbox().build_block();

    let result = session
        .with_actor(SandboxWithStaking::default_actor())
        .call::<_, Result<VotingResult, VotingError>>("end_voting", NO_ARGS, NO_ENDOWMENT)??
        .expect("end_voting failed");

    assert_eq!(result, VotingResult::For);

    Ok(())
}

mod utils {
    use drink::{
        AccountId32,
        Sandbox, session::{NO_ENDOWMENT, NO_SALT, Session},
    };

    use sandbox_with_staking::{RuntimeOrigin, RuntimeWithStaking, SandboxWithStaking};

    use crate::{BundleProvider, TestResult};

    pub fn deploy_contract(session: &mut Session<SandboxWithStaking>) -> TestResult<AccountId32> {
        Ok(session.deploy_bundle(
            BundleProvider::WeightedVoting.bundle()?,
            "new",
            &[format!("{:?}", "Test Voting")],
            NO_SALT,
            NO_ENDOWMENT,
        )?)
    }

    pub fn become_validator(session: &mut Session<SandboxWithStaking>, account: AccountId32) {
        session.sandbox().execute_with(|| {
            pallet_fake_staking::Pallet::<RuntimeWithStaking>::become_validator(
                RuntimeOrigin::signed(account),
            )
            .unwrap();
        });
    }
}
