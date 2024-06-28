# L1X Default Smart Contract

This repository contains a smart contract written in Rust for managing a counter. The contract allows for initializing, setting, incrementing, and retrieving the counter value. It utilizes the `borsh` serialization and deserialization and is designed to be deployed using the `l1x_sdk`.

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

This smart contract is designed to manage a simple counter. It provides the following functions:
- Set the counter to a specific value.
- Increment the counter.
- Retrieve the current counter value.

The contract uses the `l1x_sdk` for storage operations and `borsh` for serialization.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust, Node JS, NVM and Cargo installed. You can Set up Environment from [here](https://l1x-sdk.gitbook.io/l1x-developer-interface/v/interface-essentials/l1x-vm-sdk/l1x-native-sdk-for-l1x-vm/set-up-environment)

## Installation

Initiate a new L1X project creation process with Cargo's L1X plugin. The Default template is utilized here.
 ```sh
cargo l1x create project_name --template default
```

## Building the Contract
 ```sh
cd project_name
cargo l1x build
```
A CONTRACT_OBJECT_FILE, l1x_contract.o, would be created as target/l1x/release/l1x_contract.o

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

**Get Counter**
```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS get_counter --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Increment Counter**
```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS inc_counter --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```

**Set Counter**
```sh
 l1x-cli-beta contract call INIT_CONTRACT_ADDRESS set_counter --args '{"value":"5"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```

## License
This project is licensed under the MIT License. See the LICENSE file for more details.

