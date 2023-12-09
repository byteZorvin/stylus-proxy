    //! Example on how to interact with a deployed `stylus-hello-world` program using defaults.
//! This example uses ethers-rs to instantiate the program using a Solidity ABI.
//! Then, it attempts to check the current counter value, increment it via a tx,
//! and check the value again. The deployed program is fully written in Rust and compiled to WASM
//! but with Stylus, it is accessible just as a normal Solidity smart contract is via an ABI.

use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, U256}
};
// use eyre::eyre;
// use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;
use std::env;
use dotenv::dotenv;

// use stylus_sdk::alloy_primitives::U256;

/// Your private key file path.
// const ENV_PRIV_KEY_PATH: &str = "../.env";

/// Stylus RPC endpoint url.
// const ENV_RPC_URL: &str = "https://stylus-testnet.arbitrum.io/rpc";

/// Deployed pragram address.
// const ENV_PROGRAM_ADDRESS: &str = "0xEEA6Da9Ea2eA6D65380608349b7e957805De10B7";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();
    let priv_key = env::var("ENV_PRIV_KEY_PATH").expect("You've not set the Pvt key");
    abigen!(
        Counter,
        r#"[
            function number() external view returns (uint256)
            function setNumber(uint256 number) external
            function increment() external
        ]"#
    );

    
    let program_address = "0x280D5a75ca406c9C427aE2c3b999f8dd4C57D119";
    let rpc_url = "https://stylus-testnet.arbitrum.io/rpc";
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = program_address.parse()?;

    // let privkey = read_secret_from_file(&priv_key_path)?;
    let wallet = LocalWallet::from_str(&priv_key)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let counter = Counter::new(address, client);
    let num = counter.number().call().await;
    println!("Counter number value = {:?}", num);

    // println!("Successfully incremented counter via a tx");
    
    let _ = counter.set_number(U256::from(10)).send().await?.await?;
    println!("Successfully set counter number via a tx");
    
    let _ = counter.increment().send().await?.await?;

    let num = counter.number().call().await;
    println!("New counter number value = {:?}", num);
    Ok(())
}

// fn read_secret_from_file(fpath: &str) -> eyre::Result<String> {
//     let f = std::fs::File::open(fpath)?;
//     let mut buf_reader = BufReader::new(f);
//     let mut secret = String::new();
//     buf_reader.read_line(&mut secret)?;
//     Ok(secret.trim().to_string())
// }
