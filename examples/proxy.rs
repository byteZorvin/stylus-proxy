use dotenv::dotenv;
use ethers::{
    core::types::TransactionReceipt,
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes},
    utils::keccak256,
};
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use stylus_sdk::alloy_primitives::U256;

fn log_relayed_data(relay_data: Option<TransactionReceipt>) {
    match relay_data {
        Some(data) => {
            let data_logs_data: Bytes = data.logs[0].data.clone();
            println!("Data = {:?}", data_logs_data);
        }
        None => {
            println!("No data returned");
        }
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();
    let priv_key = env::var("ENV_PRIV_KEY_PATH").expect("You've not set the Pvt key");
    let proxy_contract_address = "0x1D16b2c1311540093c63Ab271c80331bC3C70902";
    let counter_v1_address: Address = ("0x31973Bc79631b05a3c030745391e44A9dce4B536").parse()?;
    let counter_v2_address: Address = ("0x9232F290277C97947F09B1965b207621e19a5258").parse()?;

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
            function number() external view returns (uint256)
            function setNumber(uint256 new_number) external
            function increment() external
            function decrement() external
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

    proxy
        .set_implementation(counter_v1_address)
        .send()
        .await?
        .await?;

    println!("Set implementation called successfully");

    let updated_implementation_address = proxy.get_implementation().call().await?;
    println!(
        "Updated implementation address: {:?}",
        updated_implementation_address
    );

    let number = U256::from(5u64);
    let hashed_bytes1 = keccak256("setNumber(uint256)");
    let data1 = [&hashed_bytes1[..4], &number.to_be_bytes::<32>()].concat();
    let relay_data1 = proxy.relay_to_implementation(data1).send().await?.await?;
    println!("Relayed data from set_number()");
    log_relayed_data(relay_data1);

    let hashed_bytes2 = keccak256("increment()");
    let data2 = [&hashed_bytes2[..4]].concat();
    let relay_data2 = proxy.relay_to_implementation(data2).send().await?.await?;
    println!("Relayed data from increment()");
    log_relayed_data(relay_data2);
    
    let hashed_bytes_3 = keccak256("increment()");
    let data_3 = [&hashed_bytes_3[..4]].concat();
    let relay_data3 = proxy.relay_to_implementation(data_3).send().await?.await?;
    println!("Relayed data from increment()");
    log_relayed_data(relay_data3);
    

    // Updating the Implementation to CounterV2
    proxy
        .set_implementation(counter_v2_address)
        .send()
        .await?
        .await?;

    println!("Called Set implementation successfully");

    let updated_implementation_address_v2 = proxy.get_implementation().call().await?;
    println!(
        "Updated implementation address: {:?}",
        updated_implementation_address_v2
    );

    let v2_hashed_bytes = keccak256("decrement()");
    let v2_data = [&v2_hashed_bytes[..4]].concat();
    let relayed_data_v2_dec = proxy.relay_to_implementation(v2_data).send().await?.await?;
    println!("Relayed data from decrement() in v2");
    log_relayed_data(relayed_data_v2_dec);

    
    Ok(())
}
