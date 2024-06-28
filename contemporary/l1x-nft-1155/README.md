# L1X Non-Fungible Token Smart Contract - 1155

This project implements a Non-Fungible Token (NFT-1155) contract using the L1X SDK. The contract supports minting, burning, transferring, and approving NFTs, along with various utility functions to query NFT metadata and ownership.

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

This smart contract is designed to create and manage non-fungible tokens on the L1x blockchain platform. It has below listed features:

- **Minting**: Create new NFTs. 
- **Burning**: Permanently destroy NFTs. 
- **Transferring**: Transfer ownership of NFTs between addresses. 
- **Approvals**: Approve other addresses to manage NFTs on behalf of the owner. 
- **Metadata**: Store metadata for NFTs.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust, Node JS, NVM and Cargo are installed. You can Set up Environment from [here](https://l1x-sdk.gitbook.io/l1x-developer-interface/v/interface-essentials/l1x-vm-sdk/l1x-native-sdk-for-l1x-vm/set-up-environment)

## Installation

Initiate a new L1X project creation process with Cargo's L1X plugin. The Default template is utilized here.
 ```sh
cargo l1x create project_name --template nft1155
```



## Building the Contract
 ```sh
cd project_name
cargo l1x build
```
A CONTRACT_OBJECT_FILE **l1x_nft_1155.o** would be created as target/l1x/release/l1x_nft_1155.o

## Deployment

Deploy the compiled L1X project to the L1X blockchain.

```sh
l1x-cli-beta contract deploy ./target/l1x/release/l1x_nft_1155.o --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```
You will get deployed contract address (DEPLOY_CONTRACT_ADDRESS) as the response of the above command. Use it to initialize your L1X project.

## Initialize the Contract

Initialize your deployed L1X project by setting up its base contract address.

```sh
l1x-cli-beta contract init DEPLOY_CONTRACT_ADDRESS --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000 --args '{"metadata":{"name": "NFT_TOKEN_NAME", "icon": "NFT_ICON_URL", "uri": "NFT_URI"}}'
```

On successful initialization of the project, you will get initialized contract address (INIT_CONTRACT_ADDRESS) as the response of the init command. Use it for further Readonly and State Changing Function calls.

## Usage

Here are the sample functions and how to interact with the contract:


**Mint Non-Fungible Token with ID** - State Changing Function Call

```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS nft_mint_id --args '{"to":" YOUR_WALLET_ADDRESS ","id": "NFT_ID","amount":"TOTAL_SUPPLY"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```

**Non-Fungible Token Name** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS nft_name --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Non-Fungible Token Icon** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS nft_icon --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Non-Fungible Token Uri** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS nft_uri --args '{"id":"NFT_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Non-Fungible Token Balance** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS nft_balance_of --args '{"owner": "NFT_OWNER_ADDRESS","id":"NFT_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```


**Non-Fungible Token Owned** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS nft_owned_tokens --args '{"owner": "NFT_OWNER_WALLET_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Non-Fungible Token Burn** - State Changing Function Call

```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS nft_burn --args '{"account":"NFT_OWNER_WALLET_ADDRESS","id":"NFT_ID","amount":"NFT_TO_BE_BURNED"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```

**Non-Fungible Token Set Approval for All** - State Changing Function Call

```sh

l1x-cli-beta contract call INIT_CONTRACT_ADDRESS nft_set_approval_for_all --args '{"operator": "OPERATOR_WALLET_ADDRESS","approved": true}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```

**Non-Fungible Token Is Approved for All** - State Changing Function Call

```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS nft_is_approved_for_all --args '{"owner":"NFT_OWNER_WALLET_ADDRESS","operator": "OPERATOR_WALLET_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```

**Non-Fungible Token Balance of Batch** - Read Only Function Call

```sh
l1x-cli-beta contract view 0b8ef1bf106c981f7dae8519a16d46ec62030108 nft_balance_of_batch --args '{"owners":["NFT1_OWNER_ADDRESS","NFT2_OWNER_ADDRESS"],"ids":["NFT1_ID","NFT2_ID"]}' --endpoint https://v2-testnet-rpc.l1x.foundation
```


**Non-Fungible Token Safe Transfer From** - State Changing Function Call

```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS nft_safe_transfer_from --args '{"from":"NFT_OWNER_WALLET_ADDRESS","to":"RECEIVER_WALLET_ADDRESS","id":"NFT_ID","amount":"NFT_TO_BE_TRANSFERRED","calldata":""}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```


**Non-Fungible Token Safe Batch Transfer From** - State Changing Function Call

```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS nft_safe_batch_transfer_from --args '{"from":"NFT_OWNER_WALLET_ADDRESS","to":"RECEIVER_WALLET_ADDRESS","ids":["NFT_ID"],"values":["NFT_TO_BE_TRANSFERRED"],"calldata":""}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```

## License
This project is licensed under the MIT License. See the LICENSE file for more details.
