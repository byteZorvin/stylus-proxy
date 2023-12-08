PRIVATE_KEY=$(source ./.env && echo $ENV_PRIV_KEY_PATH)
cargo stylus check

deployProxy() {
    echo "Deploying proxy"
    cargo stylus deploy --private-key $PRIVATE_KEY --wasm-file-path target/wasm32-unknown-unknown/release/proxy.wasm
    echo "Proxy deployed"

    cd src/proxy
    cargo stylus export-abi --output proxy_abi.sol
    cd -

}

deployCounter(){
    echo "Deploying Counter"
    cargo stylus deploy --private-key $PRIVATE_KEY --wasm-file-path target/wasm32-unknown-unknown/release/counter.wasm
    echo "Counter deployed"

    cd src/counter_impl
    cargo stylus export-abi --output counter_abi.sol
    cd -
}

proxy_interaction() {
    cd src/proxy
    cargo run --example interactions
    cd -
}

implementation_interaction() {
    cd src/counter_impl
    cargo run --example interactions
    cd -
}
 
# deployProxy
# deployCounter
proxy_interaction
#implementation_interaction