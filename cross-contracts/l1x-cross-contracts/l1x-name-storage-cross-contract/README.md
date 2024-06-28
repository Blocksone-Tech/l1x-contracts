# L1X Cross Contract – Name Storage

This repository contains a smart contract that makes cross contract call with other smart contract written in Rust using the L1X SDK. The contract interacts with another contract to add names.

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

The smart contract deals with cross contract call. The contract provides functions to: 
- Initialize the contract. 
- Add a name to the stored list.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust, Node JS, NVM and Cargo installed. You can Set up Environment from [here](https://l1x-sdk.gitbook.io/l1x-developer-interface/v/interface-essentials/l1x-vm-sdk/l1x-native-sdk-for-l1x-vm/set-up-environment)

## Installation

Initiate a new L1X project creation process with Cargo's L1X plugin. The Name Storage Cross Contract template is utilized here.
```sh
cargo l1x create project_name --template name-storage-cross-contract
```

## Building the Contract
 ```sh
cd project_name
cargo l1x build
```
A contract_object_file, l1x_cross_contract.o, would be created in target/l1x/release/l1x_cross_contract.o

## Deployment

Deploy the compiled L1X project to the L1X blockchain.

```sh
l1x-cli-beta contract deploy ./target/l1x/release/l1x_cross_contract.o --endpoint https://v2-testnet-rpc.l1x.foundation
```
You will get deployed contract address (DEPLOY_CONTRACT_ADDRESS) as the response of the above command. Use it to initialize your L1X project.

## Initialize the Contract

Initialize your deployed L1X project by setting up its base contract address, and provide the other L1X smart contract initialized address as argument against the contract_instance_address parameter.

```sh
l1x-cli-beta contract init DEPLOY_CONTRACT_ADDRESS --args '{"contract_instance_address":" INIT_L1X_CONTRACT_ADDRESS "}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```

On successful initialization of the project, you will get initialized contract address (INIT_CONTRACT_ADDRESS) as the response of the init command. Use it for further Readonly and State Changing Function calls.

## Usage

Here are the available functions and how to interact with the contract:

**Add Name**
```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS add_name --args '{"name":"XTalk"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```
This name would get added in the list and can be cross-checked using get_names() of L1X Contract.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.

