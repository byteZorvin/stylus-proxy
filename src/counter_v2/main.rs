
#![cfg_attr(not(feature = "export-abi"), no_main)]

// use stylus_hello_world::Proxy;
// use crate::counter::Counter;
#[cfg(feature = "export-abi")]
fn main() {
    counter_v2::main();
}

