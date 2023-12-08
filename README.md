cargo stylus deploy --wasm-file-path target/wasm32-unknown-unknown/release/counter.wasm --private-key=$pvt_key
- Address: 0x280D5a75ca406c9C427aE2c3b999f8dd4C57D119 (counter)
- Address: 0x312d3CEB4e2a3440d8E8f9B2FE0821d3e49b3f56 (proxy)



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