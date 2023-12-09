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
    utils::keccak256,
};
use stylus_sdk::alloy_primitives::U256;
use std::env;
use dotenv::dotenv;
// use eyre::eyre;
// use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();
    let priv_key = env::var("ENV_PRIV_KEY_PATH").expect("You've not set the Pvt key");
    let proxy_contract_address = "0x117693Ba99250A53BBFdC1720Ebe9C4F06fDfa9c";
    let counter_v1_address: Address = ("0x280D5a75ca406c9C427aE2c3b999f8dd4C57D119").parse()?;
    let counter_v2_address: Address = ("aldj").parse()?;


    let rpc_url = "https://stylus-testnet.arbitrum.io/rpc";
    abigen!(
        Proxy,
        r#"[
            function init(address owner) external
            function getImplementation() external view returns (address)
            function setImplementation(address implementation) external
            function relayToImplementation(uint8[] memory data) external returns (uint8[] memory)
        ]"#
    );

    abigen!(
        Counter,
        r#"[
            function number() external view returns (uint256)
            function setNumber(uint256 new_number) external
            function increment() external
            event NumberSet(uint256 number)
    ]"#
    );

    abigen!(
        CounterV2,
        r#"[
            function number() external view returns (uint256);
            function setNumber(uint256 new_number) external;
            function increment() external;
            function decrement() external;
            event NumberSet(uint256 number)
        ]"#
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = proxy_contract_address.parse()?;
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

    proxy.set_implementation(counter_v1_address).send().await?.await?;
    println!("Called Set implementation successfully");

    let implementation_address: Address = proxy.get_implementation().call().await?;
    println!(
        "Current implementation address: {:?}",
        implementation_address
    );

    let new_implementation_address: Address = ("0x2B3c8b0e5D7e6Dd5b7fD445d7e638a7FF8f0b1dA").parse()?;
    proxy.set_implementation(new_implementation_address).send().await?.await?;

    println!("Called Set implementation successfully");

    let updated_implementation_address = proxy.get_implementation().call().await?;
    println!("Updated implementation address: {:?}", updated_implementation_address);


    let number = U256::from(10u64);
    let hashed_bytes1 = keccak256("setNumber(uint256)");
    println!("Hashed bytes using keccak {:?}", hashed_bytes1);

    let data1 = [&hashed_bytes1[..4], &number.to_be_bytes::<32>()].concat();
    println!("Data: {:?}", data1.clone());

    let relay_data1 = proxy.relay_to_implementation(data1).send().await?.await?;
    println!("Relayed data from set_number(): {:?}", relay_data1);


    let hashed_bytes2 = keccak256("increment()");
    println!("Hashed bytes using keccak {:?}", hashed_bytes2);

    let data2 = [&hashed_bytes2[..4]].concat();
    println!("Data: {:?}", data2.clone());

    let relay_data2 = proxy.relay_to_implementation(data2).send().await?.await?;
    println!("Relayed data from increment(): {:?}", relay_data2);
    match relay_data2 {
        Some(data) => {
            // let log_data_int = Uint::from_le_bytes([data.logs[0].data.clone()]);
            let log_data_int = data.logs[0].data.clone();
            println!("Event log data {:?}", log_data_int);
            // let bytes_data = Bytes::from_hex("0x12")?;
            // println!("Data: {:?}", bytes_data);
            // assert!(log_data_int == bytes_data, "Not matching the event");
            // let log_data_int = u64::from_be_bytes(relay_data2.as_ref().try_into
            // ()?);
        }, 
        None => {
            println!("No data returned");
        }
    }

    let hashed_bytes_3 = keccak256("number()");
    println!("Hashed bytes using keccak {:?}", hashed_bytes_3);
    let data_3 = [&hashed_bytes_3[..4]].concat();
    println!("Data: {:?}", data_3.clone());
    let relayed_data_try = proxy.relay_to_implementation(data_3).send().await?.await?;
    println!("Relayed data try: {:?}", relayed_data_try);

    //Read event
    // let filter = Filter::new().address(address).event("NumberSet(uint256)");
    // let logs = client.get_logs(&filter).await?;
    // println!("Event log {:?}", logs);

    // let log_data_int = u64::from_be_bytes(logs[0].data.as_ref().try_into()?);
    // println!("Event log data {:?}", log_data_int);

    // let impl_addr: Address = ("0x46F4A131414E69Dde9257a6df34c1438379CABEC").parse()?;

    // let raw_call = RawCall::new().call(impl_addr, &data);

    // proxy.relay_to_implementation_try().send().await?.await?;
    // println!("Relayed data try: {:?}", relayed_data_try);


    // proxy.set_implementation(counter_v2_address).send().await?.await?;
    // println!("Called Set implementation successfully");

    // let updated_implementation_address = proxy.get_implementation().call().await?;
    // println!("Updated implementation address: {:?}", updated_implementation_address);
    
    Ok(())
}
