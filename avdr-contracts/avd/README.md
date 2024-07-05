# L1X Access Version Data Contract

This project implements AVD contracts using Rust. The contract manages access control, version control and data storage.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)

## Overview

This project handles access control, data storage and version control. It contains the features:
- Access Control Management: Request and approve permission. Request and approve data share. Check pending requests, request status and approved shares.
- Data Storage Management: Create data and check data details.
- Version Control Management: Submit and get share request.


## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust, Node JS, NVM and Cargo are installed. You can Set up Environment from [here](https://l1x-sdk.gitbook.io/l1x-developer-interface/v/interface-essentials/l1x-vm-sdk/l1x-native-sdk-for-l1x-vm/set-up-environment)

- Super, Emma and Ava are three users having their own wallet with some non-zero balance.

- Super is deploying all the contracts




## Installation

All the three contracts need to be deployed and initialized in the given sequence.

```sh
mkdir avd
cd avd
```


### 1. Types Dependency



Initiate a new L1X project creation process with Cargo's L1X plugin.
 ```sh
cargo l1x create types
```

Goto src/lib.rs and paste the smart contract there. 

Goto src/access_control_interface.rs and paste the smart contract there.

Goto src/data_storage_interface.rs and paste the smart contract there.

Similarly, paste Cargo.toml file in the project.


These contracts serve as dependency for the AVD contracts.





### 2. Access Control Installation


Initiate a new L1X project creation process with Cargo's L1X plugin.
 ```sh
cargo l1x create access_control
```

Goto src/lib.rs and paste the smart contract there. Similarly, paste Cargo.toml file in the project.


##### 2.1 Building the Access Control Contract
 ```sh
cd access_control
cargo l1x build
```
A CONTRACT_OBJECT_FILE **access_control.o** would be created as target/l1x/release/access_control.o

##### 2.2 Access Control Contract Deployment

Deploy the compiled L1X project to the L1X blockchain.

```sh
l1x-cli-beta contract deploy ./target/l1x/release/access_control.o --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```
You will get deployed contract address (ACCESS_CONTROL_DEPLOY_CONTRACT_ADDRESS) as the response of the above command. Use it to initialize your L1X project.

##### 2.3 Initialize the Access Control Contract

Initialize your deployed L1X project by setting up its base contract address.

```sh
l1x-cli-beta contract init ACCESS_CONTROL_DEPLOY_CONTRACT_ADDRESS --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000 --args '{}'
```

On successful initialization of the project, you will get initialized contract address (ACCESS_CONTROL_INIT_CONTRACT_ADDRESS) as the response of the init command. Use it for further Readonly and State Changing Access Control Function calls



### 3. Data Storage Installation


Initiate a new L1X project creation process with Cargo's L1X plugin.
 ```sh
cargo l1x create data_storage
```

Goto src/lib.rs and paste the smart contract there. Similarly, paste Cargo.toml file in the project.

##### 3.1 Building the Data Storage Contract
 ```sh
cd data_storage
cargo l1x build
```
A CONTRACT_OBJECT_FILE **data_storage.o** would be created as target/l1x/release/data_storage.o

##### 3.2 Data Storage Contract Deployment

Deploy the compiled L1X project to the L1X blockchain.

```sh
l1x-cli-beta contract deploy ./target/l1x/release/data_storage.o --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```
You will get deployed contract address (DATA_STORAGE_DEPLOY_CONTRACT_ADDRESS) as the response of the above command. Use it to initialize your L1X project.

##### 3.3 Initialize the Data Storage Contract

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


##### 3.4 Confirm the Data Storage Contract Initialization
At this stage, just check that the data storage contract is initialized successfully.
```sh
l1x-cli-beta contract view DATA_STORAGE_INIT_CONTRACT_ADDRESS owner --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

In the response, on successful initialization, you will get SUPER_WALLET_ADDRESS.


### 4. Version Control Installation


Initiate a new L1X project creation process with Cargo's L1X plugin.
 ```sh
cargo l1x create version_control
```
Goto src/lib.rs and paste the smart contract there. Similarly, paste Cargo.toml file in the project.

##### 4.1 Building the Version Control Contract
 ```sh
cd version_control
cargo l1x build
```
A CONTRACT_OBJECT_FILE **version_control.o** would be created as target/l1x/release/version_control.o

##### 4.2 Version Control Contract Deployment

Deploy the compiled L1X project to the L1X blockchain.

```sh
l1x-cli-beta contract deploy ./target/l1x/release/version_control.o --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```
You will get deployed contract address (VERSION_CONTROL_DEPLOY_CONTRACT_ADDRESS) as the response of the above command. Use it to initialize your L1X project.

##### 4.3 Initialize the Version Control Contract

Initialize your deployed L1X project by setting up its base contract address.

```sh
l1x-cli-beta contract init VERSION_CONTROL_DEPLOY_CONTRACT_ADDRESS --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000 --args '{"access_control_address":"ACCESS_CONTROL_INIT_CONTRACT_ADDRESS","cid":"DATA_STORAGE_INIT_CONTRACT_ADDRESS"}'
```

On successful initialization of the project, you will get initialized contract address (VERSION_CONTROL_INIT_CONTRACT_ADDRESS) as the response of the init command. Use it for further Readonly and State Changing Version Control Function calls



##### 4.4 Confirm the Version Control Contract Initialization
At this stage, just check that the Version Control contract is initialized successfully.
```sh
l1x-cli-beta contract view VERSION_CONTROL_INIT_CONTRACT_ADDRESS owner --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

In the response, on successful initialization, you will get SUPER_WALLET_ADDRESS.


## Usage

Here’s the sample scenario for AVD contracts along with the sample functions and how to interact with the contract.



### Scenario

- Super has Data Object
- Super gives Emma "Approve" permissions
- Ava asks Super to share this Data Object with her.
- Emma approves Ava's request


### Flow


**Request Approve Permission** - State Changing Function Call
- Step 1: Emma requests "Approve" permissions. This command is run behalf of Emma

```sh
l1x-cli-beta contract call ACCESS_CONTROL_INIT_CONTRACT_ADDRESS request_perm --args '{"cid":"DATA_STORAGE_INIT_CONTRACT_ADDRESS","perm":"Approve"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```


**Create Share Request** - State Changing Function Call
- Step 2: Ava creates a share request. This command is run behalf of Ava
- Note that the pub_key should be in base64 format

```sh
echo -n "AVA_PUB_KEY" | base64 
```
You get BASE64_AVA_PUB_KEY

```sh
l1x-cli-beta contract call ACCESS_CONTROL_INIT_CONTRACT_ADDRESS request_share --args '{"cid":"DATA_STORAGE_INIT_CONTRACT_ADDRESS","pub_key":"BASE64_AVA_PUB_KEY"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000

```


**Check pending requests** - Read Only Function Call
- Step 3: Check pending requests. This command is run behalf of Super.


```sh
l1x-cli-beta contract view ACCESS_CONTROL_INIT_CONTRACT_ADDRESS pending_requests_by_cid --args '{"cid":"DATA_STORAGE_INIT_CONTRACT_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation 
```

Note the req_idx for the pending request that is to be approved.



**Approve Permission Request** - State Changing Function Call
- Step 4: Super approves Emma's permission request. This command is run behalf of Super.

```sh
l1x-cli-beta contract call ACCESS_CONTROL_INIT_CONTRACT_ADDRESS approve_perm --args '{"req_idx":"PERMISSION_REQUEST_TO_BE_APPROVED"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```

**Check pending requests** - Read Only Function Call
- Step 5: Check pending requests again

```sh
l1x-cli-beta contract view ACCESS_CONTROL_INIT_CONTRACT_ADDRESS pending_requests_by_cid --args '{"cid":"DATA_STORAGE_INIT_CONTRACT_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```


**Create Shared Data** - State Changing Function Call


- Step 6: Super creates the shared data. This command is executed behalf of Super. Super is the owner of the data

```sh
l1x-cli-beta contract init DATA_STORAGE_DEPLOY_CONTRACT_ADDRESS --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000 --args '{"name":"DATA_NAME","description":"DATA_DESCRIPTION","pub_key":"BASE64_AVA_PUB_KEY","content":"BASE64_CONTENT"}'
```

The initialized contract address is referred to as AVA_DATA_STORAGE_INIT_ADDRESS further.


**Approve Share Request** - State Changing Function Call

- Step 7: Emma approves the share request with the just created shared data by Super. This command is executed behalf of Emma.


```sh
l1x-cli-beta contract call ACCESS_CONTROL_INIT_CONTRACT_ADDRESS approve_share --args '{"req_idx":"AVA_REQUEST_ID","shared_cid":"AVA_DATA_STORAGE_INIT_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```



**Check pending requests** - Read Only Function Call
- Step 8: Check pending requests again. This command is executed behalf of Super.

```sh
l1x-cli-beta contract view ACCESS_CONTROL_INIT_CONTRACT_ADDRESS pending_requests_by_cid --args '{"cid":"DATA_STORAGE_INIT_CONTRACT_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```


**Submit Shared Data** - State Changing Function Call
- Step 9: Super submits the just created shared data. This command is executed behalf of Super.


```sh
l1x-cli-beta contract call VERSION_CONTROL_INIT_ADDRESS submit_shared --args '{"cid":"AVA_DATA_STORAGE_INIT_ADDRESS","from_cid":"DATA_STORAGE_INIT_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000

```


**Confirm Data Sharing** - Read Only Function Call
- Step 10: Check whether the data has been shared correctly

```sh
l1x-cli-beta contract view ACCESS_CONTROL_INIT_ADDRESS approved_shares --args '{"cid":"DATA_STORAGE_INIT_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

```sh
l1x-cli-beta contract view VERSION_CONTROL_INIT_ADDRESS get_shared --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```



## License
This project is licensed under the MIT License. See the LICENSE file for more details.
