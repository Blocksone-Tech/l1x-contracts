# L1X Non-Fungible Token Smart Contract - 721

This project implements a Non-Fungible Token (NFT-721) contract using the L1X SDK. The contract supports minting, burning, transferring, and approving NFTs, along with various utility functions to query NFT metadata and ownership.

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
- **Metadata**: Store and retrieve metadata for NFTs.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust, Node JS, NVM and Cargo are installed. You can Set up Environment from [here](https://l1x-sdk.gitbook.io/l1x-developer-interface/v/interface-essentials/l1x-vm-sdk/l1x-native-sdk-for-l1x-vm/set-up-environment)

## Installation

Initiate a new L1X project creation process with Cargo's L1X plugin. The Non Fungible Token template is utilized here.
 ```sh
cargo l1x create project_name --template nft
```

## Building the Contract
 ```sh
cd project_name
cargo l1x build
```
A CONTRACT_OBJECT_FILE **l1x_nft.o** would be created as target/l1x/release/l1x_nft.o

## Deployment

Deploy the compiled L1X project to the L1X blockchain.

```sh
l1x-cli-beta contract deploy ./target/l1x/release/l1x_nft.o --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```
You will get deployed contract address (DEPLOY_CONTRACT_ADDRESS) as the response of the above command. Use it to initialize your L1X project.

## Initialize the Contract

Initialize your deployed L1X project by setting up its base contract address.

```sh
l1x-cli-beta contract init DEPLOY_CONTRACT_ADDRESS --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000 --args '{"metadata":{"name": "NFT_TOKEN_NAME","decimals": 18,"symbol": "NFT_TOKEN_SYMBOL","icon": "NFT_ICON_URL", "uri": "NFT_URI"}}'
```

On successful initialization of the project, you will get initialized contract address (INIT_CONTRACT_ADDRESS) as the response of the init command. Use it for further Readonly and State Changing Function calls.

## Usage

Here are the sample functions and how to interact with the contract:


**Mint Non-Fungible Token** - State Changing Function Call

```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS nft_mint_to --args '{"to":" YOUR_WALLET_ADDRESS "}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```


**Mint Non-Fungible Token with ID** - State Changing Function Call

```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS nft_mint_id_to --args '{"to": "YOUR_WALLET_ADDRESS ","id":"1"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```

**Non-Fungible Token Name** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS nft_name --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```


**Non-Fungible Token Symbol** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS nft_symbol --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Non-Fungible Token Decimals** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS nft_decimals --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Non-Fungible Token Icon** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS nft_icon --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Non-Fungible Token Uri** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS nft_token_uri --args '{"id":"NFT_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Non-Fungible Token MetaData** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS nft_metadata --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Non-Fungible Token Minted Total** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS nft_minted_total --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Non-Fungible Token Approve** - State Changing Function Call

```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS nft_approve --args '{"spender":"SPENDER_WALLET_ADDRESS","id":"NFT_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```

**Non-Fungible Token Transfer From** - State Changing Function Call

```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS nft_transfer_from --args '{"from":"OWNER_WALLET_ADDRESS","to": "RECEIVER_WALLET_ADDRESS","id":"NFT_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```

**Non-Fungible Token Balance** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS nft_balance_of --args '{"owner": "NFT_OWNER_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Non-Fungible Token Owner** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS nft_owner_of --args '{"id": "NFT_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```


**Non-Fungible Token Owned** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS nft_owned_tokens --args '{"owner": "NFT_OWNER_WALLET_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Non-Fungible Token Burn** - State Changing Function Call

```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS nft_burn --args '{"id":"NFT_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```

**Non-Fungible Token Set Approval for All** - State Changing Function Call

```sh

l1x-cli-beta contract call INIT_CONTRACT_ADDRESS nft_set_approval_for_all --args '{"operator": "OPERATOR_WALLET_ADDRESS","approved": true}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```


## License
This project is licensed under the MIT License. See the LICENSE file for more details.
