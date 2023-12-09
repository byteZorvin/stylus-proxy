//! Example on how to interact with a deployed stylus-hello-world program using defaults.
//! This example uses ethers-rs to instantiate the program using a Solidity ABI.
//! Then, it attempts to check the current counter value, increment it via a tx,
//! and check the value again. The deployed program is fully written in Rust and compiled to WASM
//! but with Stylus, it is accessible just as a normal Solidity smart contract is via an ABI.

use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address},
};
use stylus_sdk::function_selector;
use stylus_sdk::alloy_primitives::U256;
// use eyre::eyre;
// use std::io::{BufRead, BufReader};
use std::{str::FromStr, fmt::format};
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let proxy_contract_address = "0x312d3CEB4e2a3440d8E8f9B2FE0821d3e49b3f56";
    let rpc_url = "https://stylus-testnet.arbitrum.io/rpc";
    let priv_key = "e788f2866a5775c1e34be91f5c7b0abf92f4e79e80d5fdcdfff194ea718322cf";
    abigen!(
        Proxy,
        r#"[
            function init(address owner) external
            function getImplementation() external view returns (address)
            function setImplementation(address implementation) external
            function relayToImplementation(uint8[] memory data) external returns (uint8[] memory)
            function relayToImplementationTry() external returns (uint8[] memory)
        ]"#
    );

    abigen!(
        Counter,
        r#"[
            function number() external view returns (uint256)
            function setNumber(uint256 new_number) external
            function increment() external
    ]"#
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = proxy_contract_address.parse()?;

    // let privkey = read_secret_from_file(&priv_key_path)?;
    let wallet = LocalWallet::from_str(&priv_key)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let proxy = Proxy::new(address, client);
    let _owner_address: Address = ("0x3647fc3a4209a4b302dcf8f7bb5d58defa6b9708").parse()?;
    // proxy.init(_owner_address).send().await?.await?;
    // println!("Init successful");

    let implementation_address: Address = proxy.get_implementation().call().await?;
    println!(
        "Current implementation address: {:?}",
        implementation_address
    );

    // let new_implementation_address: Address = ("0x280D5a75ca406c9C427aE2c3b999f8dd4C57D119").parse()?;
    // proxy.set_implementation(new_implementation_address).send().await?.await?;

    // println!("Called Set implementation successfully");

    // let updated_implementation_address = proxy.get_implementation().call().await?;
    // println!("Updated implementation address: {:?}", updated_implementation_address);

    // let counter = Counter::new(implementation_address)


    let number = U256::from(10u64);
    // let selector2 = function_selector!("relayToImplementation(uint8[])");
    let selector1 = function_selector!("set_number(uint256)");
    let selector = [63, 181, 193, 203];
    // let selector_get = function_selector!("number()");
    // let selector_get = [131, 129, 245, 138]; // number()
    println!("{:?}", selector1);
    let selector3 = function_selector!("function setNumber(uint256)");
    println!("{:?}", selector3);

    let data = [
        &selector[..],
        &number.to_be_bytes::<32>()
    ]
    .concat();
    println!("Data: {:?}", data.clone());
    println!("Number: {:?}", number);
    let relay_data = proxy.relay_to_implementation(data).send().await?.await?;

    // let data2 = [
    //     &selector1[..],
    //     &selector_get[..],
    //     // &10u64.to_be_bytes(),
    // ].concat();

    // let n_res = proxy.relay_to_implementation(data2.clone()).send().await?;
    // println!("Get number called: {:?}", n_res);
    println!("Relayed data: {:?}", relay_data);

    // let impl_addr: Address = ("0x46F4A131414E69Dde9257a6df34c1438379CABEC").parse()?;

    // let raw_call = RawCall::new().call(impl_addr, &data);

    // proxy.relay_to_implementation_try().send().await?.await?;
    // println!("Relayed data try: {:?}", relayed_data_try);


    Ok(())
}
