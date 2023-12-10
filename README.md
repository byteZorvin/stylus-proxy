# StyliteProxy   : Minimal Rust Proxy with Stylus SDK

## Overview

This repository showcases a method to implement a minimal proxy for Rust contracts using the Stylus SDK. The proxy employs the `delegate_call()` function and follows the initialization pattern.
- The primary functionality allows for deploying and upgrading the implementation contract by deploying a new implementation contract and subsequently calling `set_implementation` on the proxy, specifying the address of the newly deployed contract.
- A Bash script has been included to streamline the process, automating the compilation, building, and deployment through 
- The bash calls `cargo stylus check` for before anything to make sure there are no build errors.
- After deploying the contracts using `cargo stylus deploy` it generates abi for each deployed contract and stores the Interface in a sol file using `cargo stylus export-abi`

## Usage

To utilize this project, follow these steps:

1. Clone the repository to your local environment.
2. Navigate to the repository directory.
3. Create an .env with the Private key using .env.example
4. Execute the `deploy.sh` Bash script to compile, build, and deploy the contracts automatically.

```bash
bash scripts/deploy.sh
```
5. Update your deployed address in the examples/proxy.rs
6. Run the `cargo stylus --example interactions` to run the example file

## Future Scope
### 1. Macro Integration
Future iterations of this project aim ease deploying and managing proxy upgradability, by making the a contract proxy upgradable by adding only a macro.


### 2. Integration of EIP 1967 Beacon Proxy
To implement EIP 1967 Beacon Proxy, allowing storage to be written to a specific slot. The following functions will be useful here:
```
pub unsafe fn load_bytes32(key: U256) -> B256;
pub unsafe fn store_bytes32(key: U256, data: B256);
```

### Current contract Addresses
Proxy: [0x1D16b2c1311540093c63Ab271c80331bC3C70902](https://stylus-testnet-explorer.arbitrum.io/address/0x1D16b2c1311540093c63Ab271c80331bC3C70902)
counter_v1: [0x31973Bc79631b05a3c030745391e44A9dce4B536](https://stylus-testnet-explorer.arbitrum.io/address/0x31973Bc79631b05a3c030745391e44A9dce4B536)
counter_v2: [0x9232F290277C97947F09B1965b207621e19a5258](https://stylus-testnet-explorer.arbitrum.io/address/0x9232F290277C97947F09B1965b207621e19a5258)

### Contribution
Contributions to this project are welcome! Feel free to open issues for suggestions, bugs, or enhancements.

