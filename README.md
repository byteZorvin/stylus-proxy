cargo stylus deploy --wasm-file-path target/wasm32-unknown-unknown/release/counter.wasm --private-key=$pvt_key
- Address: 0x2B3c8b0e5D7e6Dd5b7fD445d7e638a7FF8f0b1dA (counter)
- Address: 0x117693Ba99250A53BBFdC1720Ebe9C4F06fDfa9c (proxy)



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

How to get proper function selector?
HOw to parse return