# Blockchain_in_Manufacturing
_ This is a project for Blockchain Hackathon in Vietnam Blockchain Summer School made by Five Musketeers team include:
+ Lương Toàn Bách (leader)
+ Nguyễn Thị Mỹ Hạnh
+ Bùi Tấn Hải Đăng
+ Trần Quốc Nam
+ Nguyễn Hải Đăng

_ Link to Integration Tests: https://github.com/LTBach/Blockchain_in_Manufacturing_Integration_Tests
# Prerequires
+ NodeJS  
+ Near CLI  
+ Rust/Rustup and Wasm
+ Account in tesnet
# How to interact with SmartContract:
_ Clone repo.
_ cd to directory contain projects
1. Create new account in testnet
```
export CONTRACT_ID=blockchain_in_manufacturing.vbidev.testnet
export ACCOUNT_ID=uitdev.testnet
near create $CONTRACT_ID --masterAccount $ACCOUNT_ID --initialBalance 5
```

2. Build contract and Test(Unit and Integration Test)
```
cargo test 
./build.sh
cargo run --example integration-tests
```

3. Deploy and init contract
```
near deploy --wasmFile out/contract.wasm --accountId $CONTRACT_ID --initFunction new --initArgs '{"owner_id": "$ACCOUNT_ID"}'
```
