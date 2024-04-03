# Further steps

Once you have completed the previous exercises, you can try to implement the following:

1. Write a contract that allows to make a [remark](https://docs.rs/frame-system/30.0.0/frame_system/pallet/struct.Pallet.html#method.remark) in the pallet system.
   (_Should you use runtime call or chain extension?_)
2. Modify `WeightedVoting` contract to weigh voting power based on the amount of tokens staked by the voter.
   (_Hint: you can use the `stake_of` method from the `FakeStaking` pallet. But firstly, you will have to make it accessible for the contract._)\
