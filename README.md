# Blockchain_in_Manufacturing
_ This is a project for Blockchain Hackathon in Vietnam Blockchain Summer School made by Five Musketeers team include:
+ Lương Toàn Bách (leader)
+ Nguyễn Thị Mỹ Hạnh (marketing)
+ Bùi Tấn Hải Đăng
+ Nguyễn Hải Đăng

_ Link to Integration Tests: https://github.com/LTBach/Blockchain_in_Manufacturing_Integration_Tests
# Skill requires
+ NodeJS  
+ Near CLI  
+ Rust/Rustup and Wasm
+ Account in tesnet
# How to interact with SmartContract:
## 1. Prepare
_ Clone repo.  
_ cd to directory contain projects
## 2. Create new account in testnet
```
export CONTRACT_ID=blockchain_in_manufacturing.uitdev.testnet
export ACCOUNT_ID=uitdev.testnet
export CUSTOMER_ID=custome.uitdev.testnet
near create $CONTRACT_ID --masterAccount $ACCOUNT_ID --initialBalance 20
near create $CUSTOMER_ID --masterAccount $ACCOUNT_ID --initialBalance 40
```

## 3. Build contract and Test(Unit and Integration Test)
```
./build.sh
cargo test 
cargo run --example integration-tests
```

## 4. Deploy and init contract
```
near deploy --wasmFile out/contract.wasm --accountId $CONTRACT_ID --initFunction new --initArgs '{"owner_id": "$ACCOUNT_ID"}'
```

## 5. Call function
#### add_command
'''
near call manufacturing.uitdev.testnet add_command '{"command_id": "command_1", "name_product": "Iphone_14", "is_sell": true, "amount_product": "1", "price_per_product": "20000000000000000000000000", "quality": null}' --accountId $CUSTOMER_ID
'''
#### remove_command
#### get_command
#### get_product_order_way

