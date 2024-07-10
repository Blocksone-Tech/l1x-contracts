# L1X Name-Service Contract

This project implements naming service contracts using Rust.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)

## Overview

This project contains smart contracts that implement naming service logic. It contains features such as:
- Name Management: Register and Transfer Name. Change Resolver.
- Name Details: View owned names, name owner, name resolver.


## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust, Node JS, NVM and Cargo are installed. You can Set up Environment from [here](https://l1x-sdk.gitbook.io/l1x-developer-interface/v/interface-essentials/l1x-vm-sdk/l1x-native-sdk-for-l1x-vm/set-up-environment)

- Super, Ava  and Emma are the three users having their own wallet with some non-zero balance.

- Super is deploying all the contracts




## Installation

The contract need to be deployed and initialized in the given sequence.

```sh
mkdir naming_service
cd naming_service
```


### 1. Types Dependency



Initiate a new L1X project creation process with Cargo's L1X plugin.
 ```sh
cargo l1x create types
```

Goto src/lib.rs and paste the smart contract there. 

Goto src/name_service_interface.rs and paste the smart contract there.

Similarly, paste Cargo.toml file in the project.


These contracts serve as dependency for the Name-Service contracts.





### 2. Name-Service Installation


Initiate a new L1X project creation process with Cargo's L1X plugin.
 ```sh
cargo l1x create name_service
```

Goto src/lib.rs and paste the smart contract there. Similarly, paste Cargo.toml file in the project.


##### 2.1 Building the Name-Service Contract
 ```sh
cd name_service
cargo l1x build
```
A CONTRACT_OBJECT_FILE **name_service.o** would be created as target/l1x/release/name_service.o


##### 2.2 Name-Service Contract Deployment

Deploy the compiled L1X project to the L1X blockchain.

```sh
l1x-cli-beta contract deploy ./target/l1x/release/name_service.o --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```
You will get deployed contract address (NAME_SERVICE_DEPLOY_CONTRACT_ADDRESS) as the response of the above command. Use it to initialize your L1X project.


##### 2.3 Initialize the Name-Service Contract

Initialize your deployed L1X project by setting up its base contract address.

```sh
l1x-cli-beta contract init NAME_SERVICE_DEPLOY_CONTRACT_ADDRESS --args '{}'  --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```

On successful initialization of the project, you will get initialized contract address (NAME_SERVICE_INIT_CONTRACT_ADDRESS) as the response of the init command. Use it for further Readonly and State Changing Name-Service Function calls


## Usage

Here’s the sample scenario for Name-Service contracts along with the sample functions and how to interact with the contract.



### Scenario

- Ava registers a new name "name_service.l1x"
- Ava changes the name owner to Emma
- Emma changes the name resolver to her address

### Flow


**Register Root Name** - State Changing Function Call
- Step 1: Ava registers a new root name "_name_service.l1x_".

```sh
l1x-cli-beta contract call NAME_SERVICE_INIT_CONTRACT_ADDRESS register_root_name --args '{"name": "ROOT_NAME_TO_BE_REGISTERED", "resolver": "AVA_WALLET_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```


**Check Name Information** - Read Only Function Call
- Step 2: Ava checks Name Information.

- 2.1: Ava checks names owned by her. 

```sh
l1x-cli-beta contract view NAME_SERVICE_INIT_CONTRACT_ADDRESS list_owned_names --args '{"user": "AVA_WALLET_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

- 2.2: Ava checks resolver for registered name.

```sh
l1x-cli-beta contract view NAME_SERVICE_INIT_CONTRACT_ADDRESS resolve_name --args '{"name": "REGISTERED_ROOT_NAME"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

- 2.3: Ava checks owner of the registered name.

```sh
l1x-cli-beta contract view NAME_SERVICE_INIT_CONTRACT_ADDRESS owner_of --args '{"name": "REGISTERED_ROOT_NAME"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```


**Change Name Owner** - State Only Function Call
- Step 3: Ava change the registered name's owner to Emma.


```sh
l1x-cli-beta contract call NAME_SERVICE_INIT_CONTRACT_ADDRESS transfer_name --args '{"name": "REGISTERED_ROOT_NAME", "new_owner": "EMMA_WALLET_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000

```

**Check Name Information** - Read Only Function Call
- Step 4: Ava checks Name information. 

- 4.1: Ava checks names owned by her. Since, AVA has transferred the name to Emma, expected null response.

```sh
l1x-cli-beta contract view NAME_SERVICE_INIT_CONTRACT_ADDRESS list_owned_names --args '{"user": "AVA_WALLET_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```


- 4.2: Ava checks names owned by Emaa. Details of registered name _name_service.l1x_ should appear in response.

```sh
l1x-cli-beta contract view NAME_SERVICE_INIT_CONTRACT_ADDRESS list_owned_names --args '{"user": "EMMA__WALLET_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

- 4.3: Ava checks resolver for registered name. Currently, AVA is the resolver. Therefore, AVA_WALLET_ADDRESS should appear in the response.

```sh
l1x-cli-beta contract view NAME_SERVICE_INIT_CONTRACT_ADDRESS resolve_name --args '{"name": "REGISTERED_ROOT_NAME"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

- 4.4: Ava checks owner of the registered name. Now since Emma is the owner, EMMA_WALLET_ADDRESS should appear in the response.

```sh
l1x-cli-beta contract view NAME_SERVICE_INIT_CONTRACT_ADDRESS owner_of --args '{"name": "REGISTERED_ROOT_NAME"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```


**Change Name Resolver** - State Changing Function Call
- Step 5: Emma changes the resolver to her address. Only the owner of the name can do this. This command is run behalf of Emma.

```sh
l1x-cli-beta contract call NAME_SERVICE_INIT_CONTRACT_ADDRESS change_name_resolver --args '{"name": "REGISTERED_ROOT_NAME", "new_resolver": "EMMA_WALLET_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```


**Check Name Information** - Read Only Function Call
- Step 6: Emma checks Name Information.

- 6.1: Emma checks names owned by her. Details of transferred name (_name_service.l1x_) should appear in response.

```sh
l1x-cli-beta contract view NAME_SERVICE_INIT_CONTRACT_ADDRESS list_owned_names --args '{"user": "EMMA_WALLET_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

- 6.2: Emma checks resolver for registered name. Now, since Emma has changed the resolver, EMMA_WALLET_ADDRESS should appear in the response.

```sh
l1x-cli-beta contract view NAME_SERVICE_INIT_CONTRACT_ADDRESS resolve_name --args '{"name": "REGISTERED_ROOT_NAME"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

- 6.3: Emma checks owner of the registered name. EMMA_WALLET_ADDRESS should appear in the response.

```sh
l1x-cli-beta contract view NAME_SERVICE_INIT_CONTRACT_ADDRESS owner_of --args '{"name": "REGISTERED_ROOT_NAME"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```


## License
This project is licensed under the MIT License. See the LICENSE file for more details.
