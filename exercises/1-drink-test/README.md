# Testing ink! smart contracts with drink library

This is the first of our exercises, where we meet with two contracts and the drink library.

## Contracts

Firstly, let's take a look at the contracts that we will be working with.

### Enroll

The `Enroll` contract is a simple contract that allows users to enroll themselves in a public list. 
This is a paid subscription, which expires after a certain period.
Specifically, the contract has the following methods:
```rust
/// Constructs a new Enroll contract.
#[ink(constructor)]
fn new() -> Self
```

```rust
/// Enrolls the caller in the subscription list.
#[ink(message, payable, selector = 1)]
pub fn subscribe(&mut self)
```

```rust
/// Checks if `account` is an active subscriber.
#[ink(message, selector = 2)]
pub fn is_active(&self, account: AccountId) -> bool  
```

The contract crate also specifies `pub const BLOCKS_FOR_TOKEN: u32 = 10;`.
This means that for every token transferred to the `subscribe` method, the caller will be enrolled for 10 blocks.
After the paid period expires, the caller will no longer be an active subscriber.

The contract does nothing more than that.
However, it can be used in other contracts, that want to limit their functionality only to the active subscribers.

The source code of the contract can be found in the [`lib.rs`](../../contracts/enroll/lib.rs) file.
There, you will also find the integration tests written with `ink::test` macro, that utilize the off-chain engine.

### Voting

The `Voting` contract is a single use contract that allows users to vote either for or against some proposal.

The contract has the following methods:
```rust
/// Constructs a new Voting contract with the given `title`, 
/// that should describe the proposal and `enroll` account,
/// which is the address of the deployed `Enroll` contract.
#[ink(constructor)]
pub fn new(title: String, enroll: AccountId) -> Self
```

```rust
/// Opens a time window for voting. Can be called only by the contract administrator.
/// Voting is open for `deadline` blocks.
#[ink(message, selector = 1)]
pub fn start_voting(&mut self, deadline: BlockNumber) -> Result<(), VotingError>
```

```rust
/// Vote in favor of the proposal.
#[ink(message, selector = 2)]
pub fn vote_for(&mut self) -> Result<(), VotingError>
```

```rust
/// Vote against the proposal.
#[ink(message, selector = 3)]
pub fn vote_against(&mut self) -> Result<(), VotingError>
```

```rust
/// Ends the voting process and returns the result.
/// Can be called only by the contract administrator, only after the deadline.
#[ink(message, selector = 4)]
pub fn end_voting(&mut self) -> Result<VotingResult, VotingError>
```

As you can see, we depend on the `Enroll` contract - only the active subscribers are allowed to vote.

The source code of the contract can be found in the [`lib.rs`](../../contracts/voting/src/lib.rs) file.

## Drink library

Drink allows you to fully test your ink! smart contracts in a local environment, without running any real chain in the background.
More detailed description, materials and examples are available here:
- https://use.ink/basics/contract-testing/drink
- https://github.com/inkdevhub/drink
- https://github.com/inkdevhub/drink/tree/main/examples/quick-start-with-drink

## Task

In the current directory, you will find the `tests` crate with some tests written in the drink library for the `Voting` contract.

1. Run the existing tests with the following command:
```bash
cargo test --release
```

Don't forget about `--release` flag - it is a known problem that building drink tests in debug mode might cause some problems.

You should see the output similar to the following:
```bash
running 8 tests
test non_admin_cannot_start_voting ... ok
test starting_voting_emits_event ... ok
test non_admin_cannot_end_voting ... ok
test expired_subscribers_cannot_vote ... ok
test cannot_end_voting_before_deadline ... ok
test contract_deployment_works ... ok
test cannot_end_voting_before_start ... ok
test no_vote_voting ... ok
```

2. Your task is to implement the missing tests for the `Voting` contract:

```rust
/// * Deploy both `Enroll` and `Voting` contracts.
/// * Enroll the caller in the `Enroll` contract.
/// * Start the voting process.
/// * Vote for the proposal.
/// * Try voting again. Ensure that an appropriate error is returned.
#[drink::test]
fn cannot_vote_two_times(mut session: Session) -> TestResult {
    todo!("Implement test")
}

/// * Deploy both `Enroll` and `Voting` contracts.
/// * Enroll three different users in the `Enroll` contract.
/// * Start the voting process.
/// * Cast votes for the proposal from all three users.
/// * End the voting process.
/// * Ensure that the result is correct.
#[drink::test]
fn voting_works(mut session: Session) -> TestResult {
    todo!("Implement test")
}
```

_Hint: start with reading the existing tests and try to reuse as many pieces as possible._
