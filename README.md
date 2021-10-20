# `near-api-js` Starter Kit

This is a simple project to help you get started with frontend development on NEAR Protocol

## Files

```txt
├── index.html                # default page, nothing here
├── login.html                # code to login to NEAR Wallet
├── contract.html             # code to call view and change methods (basic)
└── contract-advanced.html    # code to call view and change methods (advanced)
```


# istruction

1) build the contract 
```
cargo build --target wasm32-unknown-unknown --release 
```

2)
deploy
```
cd contracts
near dev-deploy ./target/wasm32-unknown-unknown/release/donations.wasm
```

3) deployed contract 
dev-1634750735525-25016641485549

methods: 

cargo build --target wasm32-unknown-unknown --release  && near dev-deploy ./target/wasm32-unknown-unknown/release/donations.wasm

```
near call dev-1634754709855-35625310829537  new '{}' --accountId donation.testnet

near call dev-1634754709855-35625310829537  add_donation '{"amount": 100, "receiver": "some.testnet"}' --accountId donation.testnet

near call dev-1634754709855-35625310829537  add_donation '{"amount": 50, "receiver": "some1.testnet"}' --accountId donation.testnet

near view dev-1634754709855-35625310829537  get_donations

near call dev-1634754709855-35625310829537 donate '{"receiver": "some1.testnet"}' --accountId donation.testnet --amount 5
```