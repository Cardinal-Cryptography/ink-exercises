# Calling Runtime from a smart contract

In the second exercise, we will focus on the interaction between smart contracts and the runtime.

In Substrate/Polkadot ecosystem, _runtime_ is a term for the state transition function.
When a transaction (like a token transfer, or a staking operation) is executed, it is processed by the runtime, which updates the state of the blockchain.
Sometimes such a transaction is a call to a smart contract: in that case the runtime is responsible for executing the contract's code.
However, the contract might need to interact with the runtime itself, e.g., to get the current block number or to transfer some tokens.
Furthermore, the contract might want to act like a user and perform some standard runtime operations — like aforementioned token transfers or staking actions.
And this is exactly what we will focus on in the current exercise.

## Runtime

Firstly, let's take a look at one of the runtime components that we will be targetting.

### Staking pallet

The provided runtime contains a simple staking pallet: [`pallet-fake-staking`](../../runtime/pallet-fake-staking/).
It allows users to stake some tokens and to get rewards for that.
While the reward system is skipped, the staking functions are implemented.
Specifically, the pallet exposes the following functions:
```rust
/// Stakes the given amount of tokens.
pub fn stake(origin: OriginFor<T>, stake: u128)
```

```rust
/// Increase the stake by the given amount of tokens.
pub fn stake_more(origin: OriginFor<T>, more: u128)
```

The idea is that the user can start staking using the `stake` function, and later, they can add more tokens to the stake using the `stake_more` function.
However, there is one issue: minimum staking amount is 100 tokens...

## Contract

In order to allow users to stake less than 100 tokens, we will create a smart contract that will act as a proxy between the user and the staking pallet.
It will accumulate the tokens from different users until the amount is sufficient to stake them.
Once the amount is enough, the contract will call the staking pallet and stake the tokens.
After that, when a new user deposits some tokens, the contract will stake them as well (using `stake_more` method).
(Normally, the contract would also handle the rewards, but we will skip this part for now.)

Specifically, the contract has the following methods:
```rust
/// Constructs a new CommonStaking contract.
#[ink(constructor)]
fn new() -> Self
```

```rust
/// Deposits the given amount of tokens for a common stake.
#[ink(message, payable, selector = 1)]
pub fn stake(&mut self)
```

The source code of the contract is located in the [`lib.rs`](../../contracts/common-staking/src/lib.rs) file.

## Task

The current directory contains the `tests` crate with tests written in the drink library for the `CommonStaking` contract.

1. Run the existing tests with the following command:
```bash
cargo test --release
```

Some of them shall fail with similar messages:
```bash
running 6 tests
test user_can_deposit_their_stake ... ok
test stakes_more_if_new_funds_are_deposited ... FAILED
test there_is_an_actual_stake_if_the_pool_has_enough_money ... FAILED
test cumulates_stake_from_many_users_and_then_stakes ... FAILED
test there_is_no_actual_stake_if_the_pool_is_too_poor ... ok
test not_enough_is_not_enough ... ok

failures:

---- stakes_more_if_new_funds_are_deposited stdout ----
thread 'stakes_more_if_new_funds_are_deposited' panicked at lib.rs:77:5:
assertion `left == right` failed
  left: None
 right: Some(500)

---- there_is_an_actual_stake_if_the_pool_has_enough_money stdout ----
thread 'there_is_an_actual_stake_if_the_pool_has_enough_money' panicked at lib.rs:41:5:
assertion `left == right` failed
  left: None
 right: Some(100)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- cumulates_stake_from_many_users_and_then_stakes stdout ----
thread 'cumulates_stake_from_many_users_and_then_stakes' panicked at lib.rs:68:5:
assertion `left == right` failed
  left: None
 right: Some(100)
```

2. Your task is to implement the `stake` method of the `CommonStaking` contract according to the provided specification.

_Hint: you should use `RuntimeCall` enum provided in [`runtime_call.rs`](../../contracts/common-staking/src/runtime_call.rs)_
