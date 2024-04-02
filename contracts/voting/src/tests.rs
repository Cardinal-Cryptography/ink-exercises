use drink::{
    minimal::MinimalSandbox,
    Sandbox,
    sandbox_api::{balance_api::BalanceAPI, prelude::SystemAPI},
    session::{NO_ARGS, NO_ENDOWMENT, Session},
};

use utils::deploy_contracts;

use crate::{errors::VotingError, VotingResult};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;
pub const BOB: [u8; 32] = [2; 32];

#[drink::contract_bundle_provider]
enum BundleProvider {}

#[drink::test]
fn contract_deployment_works(mut session: Session) -> TestResult {
    deploy_contracts(&mut session).map(|_| ())
}

#[drink::test]
fn non_admin_cannot_start_voting(mut session: Session) -> TestResult {
    deploy_contracts(&mut session)?;

    session.set_actor(BOB.into());
    let error = session.call_and_expect_error::<_, VotingError>(
        "start_voting",
        &["10"], // deadline
        NO_ENDOWMENT,
    );
    assert_eq!(error, VotingError::NotAdmin);

    Ok(())
}

#[drink::test]
fn starting_voting_emits_event(mut session: Session) -> TestResult {
    deploy_contracts(&mut session)?;

    let contract_events_emitted = session
        .call_and("start_voting", &["10"], NO_ENDOWMENT)?
        .record()
        .last_event_batch()
        .contract_events()
        .len();

    assert_eq!(contract_events_emitted, 1);

    Ok(())
}

#[drink::test]
fn cannot_end_voting_before_start(mut session: Session) -> TestResult {
    deploy_contracts(&mut session)?;

    let error =
        session.call_and_expect_error::<_, VotingError>("end_voting", NO_ARGS, NO_ENDOWMENT);
    assert_eq!(error, VotingError::CannotEndVoting);

    Ok(())
}

#[drink::test]
fn cannot_end_voting_before_deadline(mut session: Session) -> TestResult {
    deploy_contracts(&mut session)?;

    session
        .call::<_, Result<(), VotingError>>("start_voting", &["10"], NO_ENDOWMENT)??
        .expect("start_voting failed");

    let error =
        session.call_and_expect_error::<_, VotingError>("end_voting", NO_ARGS, NO_ENDOWMENT);
    assert_eq!(error, VotingError::CannotEndVoting);

    Ok(())
}

#[drink::test]
fn non_admin_cannot_end_voting(mut session: Session) -> TestResult {
    deploy_contracts(&mut session)?;

    session
        .call::<_, Result<(), VotingError>>("start_voting", &["10"], NO_ENDOWMENT)??
        .expect("start_voting failed");

    session.sandbox().build_blocks(15);

    session.set_actor(BOB.into());
    let error =
        session.call_and_expect_error::<_, VotingError>("end_voting", NO_ARGS, NO_ENDOWMENT);
    assert_eq!(error, VotingError::NotAdmin);

    Ok(())
}

#[drink::test]
fn no_vote_voting(mut session: Session) -> TestResult {
    deploy_contracts(&mut session)?;

    session
        .call::<_, Result<(), VotingError>>("start_voting", &["10"], NO_ENDOWMENT)??
        .expect("start_voting failed");

    session.sandbox().build_blocks(15);

    let result = session
        .call::<_, Result<VotingResult, VotingError>>("end_voting", NO_ARGS, NO_ENDOWMENT)??
        .expect("end_voting failed");
    assert_eq!(result, VotingResult::Draw);

    Ok(())
}

#[drink::test]
fn expired_subscribers_cannot_vote(mut session: Session) -> TestResult {
    let (enroll_address, _) = deploy_contracts(&mut session)?;

    session.sandbox().mint_into(&BOB.into(), 1_000_000_000_000).unwrap();

    session.set_actor(BOB.into());
    session
        .call_with_address::<_, ()>(
            enroll_address,
            "subscribe",
            NO_ARGS,
            Some(1), // this will be enough for 10 blocks
        )??;

    session.sandbox().build_blocks(15);

    session.set_actor(MinimalSandbox::default_actor());
    session
        .call::<_, Result<(), VotingError>>("start_voting", &["10"], NO_ENDOWMENT)??
        .expect("start_voting failed");

    session.set_actor(BOB.into());
    let error = session.call_and_expect_error::<_, VotingError>("vote_for", NO_ARGS, NO_ENDOWMENT);
    assert_eq!(error, VotingError::NotAuthorized);

    Ok(())
}

#[drink::test]
fn cannot_vote_two_times(mut session: Session) -> TestResult {
    todo!("Implement test")
}

#[drink::test]
fn voting_works(mut session: Session) -> TestResult {
    todo!("Implement test")
}

mod utils {
    use drink::{
        AccountId32,
        minimal::MinimalSandbox,
        session::{NO_ARGS, NO_ENDOWMENT, NO_SALT, Session},
    };

    use crate::tests::{BundleProvider, TestResult};

    pub fn deploy_contracts(
        session: &mut Session<MinimalSandbox>,
    ) -> TestResult<(AccountId32, AccountId32)> {
        let enroll_address = session.deploy_bundle(
            BundleProvider::Enroll.bundle()?,
            "new",
            NO_ARGS,
            NO_SALT,
            NO_ENDOWMENT,
        )?;

        let voting_address = session.deploy_bundle(
            BundleProvider::Voting.bundle()?,
            "new",
            &[format!("{:?}", "Test Voting"), format!("{enroll_address}")],
            NO_SALT,
            NO_ENDOWMENT,
        )?;

        Ok((enroll_address, voting_address))
    }
}
