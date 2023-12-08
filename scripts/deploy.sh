PRIVATE_KEY=$(source ./.env && echo $ENV_PRIV_KEY_PATH)
echo $PRIVATE_KEY
# cargo stylus check

deploy() {
    cargo stylus deploy --private-key $PRIVATE_KEY --wasm-file-path target/wasm32-unknown-unknown/release/proxy.wasm
    echo "Proxy deployed"

    cargo stylus deploy --private-key $PRIVATE_KEY --wasm-file-path target/wasm32-unknown-unknown/release/counter.wasm
    echo "Counter deployed"

    cd src/proxy
    cargo stylus export-abi --output proxy_abi.sol
    cd -

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
 
# deploy
proxy_interaction
# implementation_interaction