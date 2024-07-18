# L1X Profile Card Contracts

This project implements profile card contracts using Rust.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)

## Overview

This project contains smart contracts that implement profile card logic. It contains features such as:
- Name Service: Register name, check resolver and owner of name.
- Name NFT: Mint, approve and check NFT owner.
- Provatar NFT: Mint, approve and check NFT owner.
- Profile Card: Add, transfer and list Name.
- Data Storage: Store data.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust, Node JS, NVM and Cargo are installed. You can Set up Environment from [here](https://l1x-sdk.gitbook.io/l1x-developer-interface/v/interface-essentials/l1x-vm-sdk/l1x-native-sdk-for-l1x-vm/set-up-environment)

- Super, Ava  and Emma are the three users having their own wallet with some non-zero balance.

- Super is deploying and initializing all the contracts except Profile Card contract.

- Ava is deploying and initializing the Profile card contract.




## Installation

The contract need to be deployed and initialized in the given sequence.

```sh
mkdir profile_contracts
cd profile_contracts
```


### 1. Types Dependency



Initiate a new L1X project creation process with Cargo's L1X plugin.
 ```sh
cargo l1x create types
```

Goto src/lib.rs and paste the smart contract there. 

Goto src/data_storage_interface.rs and paste the smart contract there.
Goto src/name_service_interface.rs and paste the smart contract there.
Goto src/nft_interface.rs and paste the smart contract there.

Similarly, paste Cargo.toml file in the project.


These contracts serve as dependency for the Profile Card contracts.





### 2.  Name-Service Installation


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



### 3. Name NFT Installation


Initiate a new L1X project creation process with Cargo's L1X plugin.
 ```sh
cargo l1x create name_nft
```

Goto src/lib.rs and paste the smart contract there. Similarly, paste Cargo.toml file in the project.


##### 3.1 Building the Name NFT Contract
 ```sh
cd name_nft
cargo l1x build
```
A CONTRACT_OBJECT_FILE **name_nft.o** would be created as target/l1x/release/name_nft.o

##### 3.2 Name NFT Contract Deployment

Deploy the compiled L1X project to the L1X blockchain.

```sh
l1x-cli-beta contract deploy ./target/l1x/release/name_nft.o --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```
You will get deployed contract address (NAME_NFT_DEPLOY_CONTRACT_ADDRESS) as the response of the above command. Use it to initialize your L1X project.

##### 3.3 Initialize the Name NFT Contract

Initialize your deployed L1X project by setting up its base contract address.

```sh
l1x-cli-beta contract init NAME_NFT_DEPLOY_CONTRACT_ADDRESS --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000 --args '{"name_service_address":"NAME_SERVICE_INIT_CONTRACT_ADDRESS", "metadata": {"name":"NAME_SERVICE_NFT_NAME", "decimals":0, "symbol":"NAME_SERVICE_NFT_SYMBOL"}}'
```

On successful initialization of the project, you will get initialized contract address (NAME_NFT_INIT_CONTRACT_ADDRESS) as the response of the init command. Use it for further Readonly and State Changing Name NFT Function calls


##### 3.4 Confirm the Name NFT Contract Initialization
At this stage, just check that the Name NFT contract is initialized successfully.

```sh
l1x-cli-beta contract view NAME_NFT_INIT_CONTRACT_ADDRESS nft_name --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```
In the response, on successful initialization, you will get NAME_SERVICE_NFT_NAME used during Name NFT Contract initialization.


### 4. Provatar NFT Installation


Initiate a new L1X project creation process with Cargo's L1X plugin.
 ```sh
cargo l1x create provatar_nft
```

Goto src/lib.rs and paste the smart contract there. Similarly, paste Cargo.toml file in the project.

##### 4.1 Building the Provatar NFT Contract
 ```sh
cd provatar_nft
cargo l1x build
```
A CONTRACT_OBJECT_FILE **provatar_nft.o** would be created as target/l1x/release/provatar_nft.o

##### 4.2 Provatar NFT Contract Deployment

Deploy the compiled L1X project to the L1X blockchain.

```sh
l1x-cli-beta contract deploy ./target/l1x/release/provatar_nft.o --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```
You will get deployed contract address (PROVATAR_NFT_DEPLOY_CONTRACT_ADDRESS) as the response of the above command. Use it to initialize your L1X project.

##### 4.3 Initialize the Provatar NFT Contract

Initialize your deployed L1X project by setting up its base contract address.


```sh
l1x-cli-beta contract init PROVATAR_NFT_DEPLOY_CONTRACT_ADDRESS --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000 --args '{"metadata": {"name":"PROVATAR_NFT_NAME", "decimals":0, "symbol":"PROVATAR_NFT_SYMBOL", "icon":"PROVATAR_NFT_ICON"}}'
```

On successful initialization of the project, you will get initialized contract address (PROVATAR_NFT_INIT_CONTRACT_ADDRESS) as the response of the init command. Use it for further Readonly and State Changing Provatar NFT Function calls


##### 4.4 Confirm the Provatar NFT Contract Initialization
At this stage, just check that the Provatar NFT contract is initialized successfully.

```sh
l1x-cli-beta contract view PROVATAR_NFT_INIT_CONTRACT_ADDRESS nft_name --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```
In the response, on successful initialization, you will get PROVATAR_NFT_NAME used during Provatar NFT Contract initialization.


### 5. Profile Card Installation

**Note that this contract is deployed and initialized by Ava.**
Initiate a new L1X project creation process with Cargo's L1X plugin.
 ```sh
cargo l1x create profile_card
```
Goto src/lib.rs and paste the smart contract there. Similarly, paste Cargo.toml file in the project.

##### 5.1 Building the Profile Card Contract
 ```sh
cd profile_card
cargo l1x build
```
A CONTRACT_OBJECT_FILE **profile_card.o** would be created as target/l1x/release/profile_card.o

##### 5.2 Profile Card Contract Deployment

Deploy the compiled L1X project to the L1X blockchain.

```sh
l1x-cli-beta contract deploy ./target/l1x/release/profile_card.o --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```
You will get deployed contract address (PROFILE_CARD_DEPLOY_CONTRACT_ADDRESS) as the response of the above command. Use it to initialize your L1X project.

##### 5.3 Initialize the Profile Card Contract

Initialize your deployed L1X project by setting up its base contract address.

```sh
l1x-cli-beta contract init PROFILE_CARD_DEPLOY_CONTRACT_ADDRESS --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000 --args '{"metadata": {"name":PROFILE_CARD_NAME", "description":"PROFILE_CARD_DESCRIPTION"}}'
```

On successful initialization of the project, you will get initialized contract address (PROFILE_CARD_INIT_CONTRACT_ADDRESS) as the response of the init command. Use it for further Readonly and State Changing Profile Card Function calls

##### 5.4 Confirm the Profile Card Contract Initialization
At this stage, just check that the Profile Card contract is initialized successfully.

```sh
l1x-cli-beta contract view PROFILE_CARD_INIT_CONTRACT_ADDRESS metadata --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```
In the response, on successful initialization, you will get metadata used during Profile Card Contract initialization.


### 6. Data Storage Installation


Initiate a new L1X project creation process with Cargo's L1X plugin.
 ```sh
cargo l1x create data_storage
```

Goto src/lib.rs and paste the smart contract there. Similarly, paste Cargo.toml file in the project.

##### 6.1 Building the Data Storage Contract
 ```sh
cd data_storage
cargo l1x build
```
A CONTRACT_OBJECT_FILE **data_storage.o** would be created as target/l1x/release/data_storage.o

##### 6.2 Data Storage Contract Deployment

Deploy the compiled L1X project to the L1X blockchain.

```sh
l1x-cli-beta contract deploy ./target/l1x/release/data_storage.o --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```
You will get deployed contract address (DATA_STORAGE_DEPLOY_CONTRACT_ADDRESS) as the response of the above command. Use it to initialize your L1X project.

##### 6.3 Initialize the Data Storage Contract

Initialize your deployed L1X project by setting up its base contract address.
- Note that the pub_key and content needs to be passed in base64 format. Use below command to get base64 format.

```sh
echo -n "SUPER_PUB_KEY" | base64 
```
You get BASE64_SUPER_PUB_KEY


```sh
echo -n "CONTENT" | base64 
```
You get BASE64_CONTENT

```sh
l1x-cli-beta contract init DATA_STORAGE_DEPLOY_CONTRACT_ADDRESS --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000 --args '{"name":"DATA_NAME","description":"DATA_DESCRIPTION","pub_key":"BASE64_SUPER_PUB_KEY","content":"BASE64_CONTENT"}'
```

On successful initialization of the project, you will get initialized contract address (DATA_STORAGE_INIT_CONTRACT_ADDRESS) as the response of the init command. Use it for further Readonly and State Changing Data Storage Function calls


##### 6.4 Confirm the Data Storage Contract Initialization
At this stage, just check that the data storage contract is initialized successfully.
```sh
l1x-cli-beta contract view DATA_STORAGE_INIT_CONTRACT_ADDRESS owner --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

In the response, on successful initialization, you will get SUPER_WALLET_ADDRESS.



## Usage

Here’s the sample scenario for Profiling Service contracts along with the sample functions and how to interact with the contract.



### Scenario

- Ava mints Name NFT with "ava.l1x".
- Ava mints Provatar NFT
- Ava creates Profile Card with the minted NFTs
- Ava transfers Name NFT that is attached to Profile Card to Emma
- Provatar NFT is transferred automatically

### Flow

**Mint Name NFT** - State Changing Function Call
- Step 1: Ava mints Name NFT "ava.l1x".

1.1: Ava mints Name NFT.

```sh
l1x-cli-beta contract call NAME_NFT_INIT_CONTRACT_ADDRESS nft_mint_to --args '{"to":"AVA_WALLET_ADDRESS","name": "ROOT_NAME_TO_BE_REGISTERED"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```

1.2: Ava gets ID of minted Name NFT.

```sh
l1x-cli-beta contract view NAME_NFT_INIT_CONTRACT_ADDRESS nft_owned_tokens --args '{"owner": "AVA_WALLET_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```
Save it as NAME_NFT_ID as used in later commands.


**Check Name Information** - Read Only Function Call
- Step 2: Ava checks Name Information.

2.1: Ava checks resolver for registered name. Response should be NFT Owner address i.e. AVA's wallet address.

```sh
l1x-cli-beta contract view NAME_SERVICE_INIT_CONTRACT_ADDRESS resolve_name --args '{"name": "REGISTERED_ROOT_NAME"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

2.2: Ava checks owner of the registered name. Owner should be NAME_NFT_INIT_CONTRACT_ADDRESS
```sh
l1x-cli-beta contract view NAME_SERVICE_INIT_CONTRACT_ADDRESS owner_of --args '{"name": "REGISTERED_ROOT_NAME"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```


**Mint Provatar NFT** - State Changing Function Call
- Step 3: Ava mints Provatar NFT. The minted NFT will refer to the Data Object instance.

3.1: Ava mints Provatar NFT

```sh
l1x-cli-beta contract call PROVATAR_NFT_INIT_CONTRACT_ADDRESS nft_mint_to --args '{"to":"AVA_WALLET_ADDRESS","cid": "DATA_STORAGE_INIT_CONTRACT_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000

```
3.2: Ava gets ID of minted Provatar NFT

```sh
l1x-cli-beta contract view PROVATAR_NFT_INIT_CONTRACT_ADDRESS nft_owned_tokens --args '{"owner": "AVA_WALLET_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```
Save it as PROVATAR_NFT_ID as used in later commands.


**Add Name NFT and Provatar NFT** - State Changing Function Call
- Step 4: Ava adds the minted Name NFT and Provatar NFT to Profile. Before adding, Ava needs to approve "spender" in NFTs. The spender is the PROFILE_CARD_INIT_CONTRACT_ADDRESS.

4.1: Ava approves Spender in Provatar NFT.

```sh
l1x-cli-beta contract call PROVATAR_NFT_INIT_CONTRACT_ADDRESS nft_approve --args '{"spender":"PROFILE_CARD_INIT_CONTRACT_ADDRESS","id": "PROVATAR_NFT_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000

```


4.2: Ava approves Spender in Name NFT.

```sh
l1x-cli-beta contract call NAME_NFT_INIT_CONTRACT_ADDRESS nft_approve --args '{"spender":"PROFILE_CARD_INIT_CONTRACT_ADDRESS","id": "NAME_NFT_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```

4.3: Ava adds Name NFT and Provatar NFT to the Profile Card.


```sh
l1x-cli-beta contract call PROFILE_CARD_INIT_CONTRACT_ADDRESS add_name --args '{"name_nft":{"address":"NAME_NFT_INIT_CONTRACT_ADDRESS","id":"NAME_NFT_ID"}, "provatar_nft":{"address":"PROVATAR_NFT_INIT_CONTRACT_ADDRESS","id": "PROVATAR_NFT_ID"}}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```

**Check Name Information** - Read Only Function Call
- Step 5: Ava checks Name Information.

5.1: Ava checks resolver for registered name. Response should be NFT Owner address i.e. PROFILE_CARD_INIT_CONTRACT_ADDRESS.

```sh
l1x-cli-beta contract view NAME_SERVICE_INIT_CONTRACT_ADDRESS resolve_name --args '{"name": "REGISTERED_ROOT_NAME"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

5.2: Ava checks owner of the registered name. Owner should be NAME_NFT_INIT_CONTRACT_ADDRESS
```sh
l1x-cli-beta contract view NAME_SERVICE_INIT_CONTRACT_ADDRESS owner_of --args '{"name": "REGISTERED_ROOT_NAME"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

5.3: Ava checks name in the Profile Card. REGISTERED_ROOT_NAME, NAME_NFT and PROVATAR_NFT details get reflected as response.
```sh
l1x-cli-beta contract view PROFILE_CARD_INIT_CONTRACT_ADDRESS list_names --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Transfer Name** - State Changing Function Call
- Step 6: Ava transfers the added name to Emma

```sh
l1x-cli-beta contract call PROFILE_CARD_INIT_CONTRACT_ADDRESS transfer_name --args '{"name": "REGISTERED_ROOT_NAME", "to": "EMMA_WALLET_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```


**Check Name Information** - Read Only Function Call
- Step 7: Ava checks Name Information.


7.1: Ava checks resolver for registered name. EMMA_WALLET_ADDRESS should appear in the response.

```sh
l1x-cli-beta contract view NAME_SERVICE_INIT_CONTRACT_ADDRESS resolve_name --args '{"name": "REGISTERED_ROOT_NAME"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

7.2: Ava checks owner of the registered name. NAME_NFT_INIT_CONTRACT_ADDRESS should appear in the response.

```sh
l1x-cli-beta contract view NAME_SERVICE_INIT_CONTRACT_ADDRESS owner_of --args '{"name": "REGISTERED_ROOT_NAME"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

7.3: Ava checks Name NFT owner. EMMA_WALLET_ADDRESS should appear in the response.

```sh
l1x-cli-beta contract view NAME_NFT_INIT_CONTRACT_ADDRESS nft_owner_of --args '{"id": "NAME_NFT_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

7.4: Ava checks Provatar NFT owner. EMMA_WALLET_ADDRESS should appear in the response.

```sh
l1x-cli-beta contract view PROVATAR_NFT_INIT_CONTRACT_ADDRESS nft_owner_of --args '{"id": "PROVATAR_NFT_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

7.5: Ava checks names in Profile Card. The list should be empty now.

```sh
l1x-cli-beta contract view PROFILE_CARD_INIT_CONTRACT_ADDRESS list_names --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

## License
This project is licensed under the MIT License. See the LICENSE file for more details.
