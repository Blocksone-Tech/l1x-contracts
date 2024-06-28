# L1X Contract – Name Storage

This repository contains a smart contract written in Rust using the L1X SDK. The contract allows users to store names and retrieve them.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Building the Contract](#building-the-contract)
- [Deployment](#deployment)
- [Initialize the Contract](#initialize-the-contract)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Overview

The smart contract deals with name storage and retrieval. The contract provides functions to: 
- Initialize the contract. 
- Add a name to the stored list. 
- Retrieve the list of stored names. 

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust, Node JS, NVM and Cargo installed. You can Set up Environment from [here](https://l1x-sdk.gitbook.io/l1x-developer-interface/v/interface-essentials/l1x-vm-sdk/l1x-native-sdk-for-l1x-vm/set-up-environment)

## Installation

Initiate a new L1X project creation process with Cargo's L1X plugin.
```sh
cargo l1x create project_name
```

Goto src/lib.rs and paste the smart contract there.
Similarly, paste Cargo.toml file in the project.

## Building the Contract
 ```sh
cd project_name
cargo l1x build
```
A contract_object_file, l1x_contract.o, would be created in target/l1x/release/l1x_contract.o

## Deployment

Deploy the compiled L1X project to the L1X blockchain.

```sh
l1x-cli-beta contract deploy ./target/l1x/release/l1x_contract.o --endpoint https://v2-testnet-rpc.l1x.foundation
```
You will get deployed contract address (DEPLOY_CONTRACT_ADDRESS) as the response of the above command. Use it to initialize your L1X project.

## Initialize the Contract

Initialize your deployed L1X project by setting up its base contract address.

```sh
l1x-cli-beta contract init DEPLOY_CONTRACT_ADDRESS --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000 --args '{}'
```

On successful initialization of the project, you will get initialized contract address (INIT_CONTRACT_ADDRESS) as the response of the init command. Use it for further Readonly and State Changing Function calls.

## Usage

Here are the available functions and how to interact with the contract:

**Add Name**
```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS add_name --args '{"name":"LayerOneX"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```

**Get Names**
```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS get_names --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

## License
This project is licensed under the MIT License. See the LICENSE file for more details.

