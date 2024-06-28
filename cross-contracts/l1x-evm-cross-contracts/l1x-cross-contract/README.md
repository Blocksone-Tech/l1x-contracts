# L1X EVM CROSS CONTRACT

This repository contains an L1X smart contract to make cross contract call to L1X EVM smart contract. The contract allows users to set and get value.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Building the Contract](#building-the-contract)
- [Deployment](#deployment)
- [Initialize the Contract](#initialize-the-contract)
- [Usage](#usage)
- [License](#license)

## Overview

The smart contract deals with cross contract call to an evm smart contract. The contract provides ERC20 functions

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust, Node JS, NVM and Cargo installed. You can Set up Environment from [here](https://l1x-sdk.gitbook.io/l1x-developer-interface/v/interface-essentials/l1x-vm-sdk/l1x-native-sdk-for-l1x-vm/set-up-environment)

## Installation

Initiate a new L1X project creation process with Cargo's L1X plugin. The L1X EVM Cross Contract template is utilized here.
```sh
cargo l1x create project_name --template l1x-evm-cross-contract
```

## Building the Contract
 ```sh
cd project_name
cargo l1x build
```
A contract_object_file, l1x_evm_cross_contract.o, would be created in target/l1x/release/l1x_evm_cross_contract.o

## Deployment

Deploy the compiled L1X project to the L1X blockchain.

```sh
l1x-cli-beta contract deploy ./target/l1x/release/l1x_evm_cross_contract.o --endpoint https://v2-testnet-rpc.l1x.foundation
```
You will get deployed contract address (DEPLOY_CONTRACT_ADDRESS) as the response of the above command. Use it to initialize your L1X project.

## Initialize the Contract

Initialize your deployed L1X project by setting up its base contract address.

```sh
l1x-cli-beta contract init DEPLOY_CONTRACT_ADDRESS --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000 --args '{"evm_address":"L1X_EVM_CONTRACT_ADDRESS"}'
```

On successful initialization of the project, you will get initialized contract address (INIT_CONTRACT_ADDRESS) as the response of the init command. Use it for further Readonly and State Changing Function calls.


## Usage

Here are the available functions and how to interact with the contract:

**Set Value** - State Changing Function Call
```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS set_value --args '{"data":"INTEGER_VALUE"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000

```


**Get Value** - Read Only Function Call
```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS get_value --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation

```


## License
This project is licensed under the MIT License. See the LICENSE file for more details.

