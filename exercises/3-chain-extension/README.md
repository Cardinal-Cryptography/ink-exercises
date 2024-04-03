# Calling chain extension

In the previous exercise, we have learned how to dispatch standard runtime transactions from a smart contract.
However, the runtime may provide dedicated functions for smart contracts, which are not available for regular transactions, i.e., for humar users.
These functions are called _chain extensions_.

They are used to provide smart contracts with additional capabilities, like access to the runtime environment, or to allow them to interact with other pallets in a more controlled way.
What is more, in contrast to the call-runtime mechanism, chain extensions can actually return a value to the contract.

## Runtime

We will be working with the same runtime as in the previous exercise, i.e., the one exposing the `pallet-fake-staking`.
However, this time we will be using the chain extension mechanism to interact with the staking pallet in order to fetch some information about current validators.

### Staking pallet

Apart from the `stake` and `stake_more` functions (that will be of no interest to us in this exercise), the staking pallet exposes some functions related to becoming a validator:
```rust
/// Become a validator. Just like that.
/// 
/// This is a public operation that can be called by anyone as a standard blockchain transaction.
pub fn become_validator(origin: OriginFor<T>)
```

```rust
/// Check if the given account is a validator.
/// 
/// This method is internal for the inner workings of the runtime.
pub fn is_validator(account: <T as frame_system::Config>::AccountId) -> bool
```

### Chain extension

The runtime provides a chain extension that allows smart contracts to check if a given account is a validator.
Chain extension has two parts:
- the [runtime side](../../runtime/sandbox-with-staking/chain_extension.rs) implementation, that delegates query to the staking pallet
- the [contract side](../../contracts/weighted-voting/src/chain_extension.rs), that describes to the contract how to call the chain extension

The API that we are interested in is defined as:
```rust
fn is_validator(account: AccountId) -> bool
```

## Contract

The contract that we will be working with is the `WeightedVoting` contract.
It is pretty similar to the one that we were playing with in the first exercise, but there are two changes:
- we do not use any enrollment mechanism, so anyone can vote
- the voting power of a voter depends on whether they are a validator or not: check `STANDARD_VOTE` and `VALIDATOR_VOTE` in the contract code

The source code of the contract is located in the [`lib.rs`](../../contracts/weighted-voting/src/lib.rs) file.

## Task

The current directory contains the `tests` crate with tests written in the drink library for the `WeightedVoting` contract.

1. Run the existing tests with the following command:
```bash
cargo test --release
```

Some of them shall fail with similar messages:
```bash
running 3 tests
test standard_voting_works ... ok
test validator_overvotes_others ... FAILED
test standard_voting_works_multiple_actors ... ok

failures:

---- validator_overvotes_others stdout ----
thread 'validator_overvotes_others' panicked at lib.rs:103:5:
assertion `left == right` failed
  left: Against
 right: For
```

2. Your task is to implement the `vote` method of the `WeightedVoting` contract according to the provided specification.
