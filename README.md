# Rust Smart Contract Template

## Getting started

To get started with this template:

1. Click the "Use this template" button to create a new repo based on this template
2. Update line 2 of `Cargo.toml` with your project name
3. Update line 4 of `Cargo.toml` with your project author names
4. Set up the [prerequisites](https://github.com/near/near-sdk-rs#pre-requisites)
5. Begin writing your smart contract in `src/lib.rs`
6. Test the contract 

    `cargo test -- --nocapture`

8. Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`

**Get more info at:**

* [Rust Smart Contract Quick Start](https://docs.near.org/docs/develop/contracts/rust/intro)
* [Rust SDK Book](https://www.near-sdk.io/)

### Advanced version of escrow which allows vesting and linear release is in development for a dex

near call escrowtrial.testnet deposits_of '{"payee":"escrowtrial.testnet"}' --accountId $ID
near call escrowtrial.testnet withdraw '{"payee": "jobaftt.testnet","amount": "10000000000000"}' --accountId $ID of withdrawer only fucking
near call escrowtrial.testnet deposit '{"payee": "jobaftt.testnet","amount": "10000000000000"}' --accountId $ID --amount 100
