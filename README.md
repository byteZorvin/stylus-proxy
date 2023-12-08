cargo stylus deploy --wasm-file-path target/wasm32-unknown-unknown/release/counter.wasm --private-key=$pvt_key
- Address: 0x46F4A131414E69Dde9257a6df34c1438379CABEC (counter)
- Address: 0xF6429BB496c03950a5E335bF4Dfa34A6b9f9169e (proxy)



How to interact with the contract from cli?
How to write tests locally in stylus sdk?   

let selector1 = function_selector!("relayToImplementation(uint8[] memory data)");
let selector2 = function_selector!("setNumber(uint256 new_number) external");
let data = [
    &selector1[..],
    &selector2[..],
    &msg::sender().into_array(),
    &self.asset.get().into_array(),
    &amount.to_be_bytes::<32>(),
].concat();