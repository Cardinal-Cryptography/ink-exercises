#![cfg(test)]

use drink::{
    sandbox_api::system_api::SystemAPI,
    session::{NO_ARGS, NO_ENDOWMENT, Session},
};

use sandbox_with_staking::SandboxWithStaking;
use voting::{errors::VotingError, VotingResult};

use crate::utils::deploy_contracts;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;
type ContractResult<T = ()> = Result<T, VotingError>;

#[drink::contract_bundle_provider]
enum BundleProvider {}

#[drink::test(sandbox = SandboxWithStaking)]
fn standard_account_can_vote(mut session: Session) -> TestResult {
    let (enroll_address, _) = deploy_contracts(&mut session)?;

    session.call_with_address::<_, ()>(enroll_address, "subscribe", NO_ARGS, Some(1))??;

    session
        .call::<_, ContractResult>("start_voting", &["10"], NO_ENDOWMENT)??
        .expect("start_voting failed");

    session
        .call::<_, ContractResult>("vote_for", NO_ARGS, NO_ENDOWMENT)??
        .expect("vote_for failed");

    session.sandbox().build_blocks(20);

    let result = session
        .call::<_, ContractResult<VotingResult>>("end_voting", NO_ARGS, NO_ENDOWMENT)??
        .expect("end_voting failed");
    assert_eq!(result, VotingResult::For);

    Ok(())
}

mod utils {
    use drink::{
        AccountId32,
        session::{NO_ARGS, NO_ENDOWMENT, NO_SALT, Session},
    };

    use sandbox_with_staking::SandboxWithStaking;

    use crate::{BundleProvider, TestResult};

    pub fn deploy_contracts(
        session: &mut Session<SandboxWithStaking>,
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
