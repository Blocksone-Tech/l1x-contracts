# L1X Fungible Token Smart Contract

This project implements a simple fungible token contract using Rust. The contract manages token balances, allowances, and metadata, enabling minting, transferring, and approving tokens. It also includes an authorization mechanism for certain operations.

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

This smart contract is designed to create and manage fungible tokens on the L1x blockchain platform. It has below listed features:

- **Metadata Management**: Store and retrieve token metadata such as name, symbol, decimals, and icon.
- **Minting**: Authorized callers can mint new tokens.
- **Transfers**: Users can transfer tokens to other accounts.
- **Allowance Management**: Users can approve, increase, or decrease allowances for other accounts to spend on their behalf.
- **Authorization**: Only the contract owner can add authorized callers who can perform specific actions.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust, Node JS, NVM and Cargo are installed. You can Set up Environment from [here](https://l1x-sdk.gitbook.io/l1x-developer-interface/v/interface-essentials/l1x-vm-sdk/l1x-native-sdk-for-l1x-vm/set-up-environment)

## Installation

Initiate a new L1X project creation process with Cargo's L1X plugin. The Default template is utilized here.
 ```sh
cargo l1x create project_name --template ft
```

## Building the Contract
 ```sh
cd project_name
cargo l1x build
```
A CONTRACT_OBJECT_FILE **l1x_ft.o** would be created as target/l1x/release/l1x_ft.o

## Deployment

Deploy the compiled L1X project to the L1X blockchain.

```sh
l1x-cli-beta contract deploy ./target/l1x/release/l1x_ft.o --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```
You will get deployed contract address (DEPLOY_CONTRACT_ADDRESS) as the response of the above command. Use it to initialize your L1X project.

## Initialize the Contract

Initialize your deployed L1X project by setting up its base contract address.

```sh
l1x-cli-beta contract init DEPLOY_CONTRACT_ADDRESS --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000 --args '{"metadata":{"name": "YOUR_TOKEN_NAME","decimals": 18,"symbol": "YOUR_TOKEN_SYMBOL", "icon": ""},"account_ids":["YOUR_WALLET_ADDRESS"],"amounts":["TOTAL_SUPPLY"]}'
```

On successful initialization of the project, you will get initialized contract address (INIT_CONTRACT_ADDRESS) as the response of the init command. Use it for further Readonly and State Changing Function calls. YOUR_WALLET_ADDRESS serves as the fungible token OWNER_WALLET_ADDRESS in subsequent examples.

## Usage

Here are the sample functions and how to interact with the contract:


**Mint Fungible Token** - State Changing Function Call

```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS ft_mint --args '{"recipient_id":"OWNER_WALLET_ADDRESS","amount":"FT_TO_BE_MINTED"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000

```

**Fungible Token Balance** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS ft_balance_of --args '{"account_id":" OWNER_WALLET_ADDRESS "}' --endpoint https://v2-testnet-rpc.l1x.foundation
```
**Fungible Token Name** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS ft_name --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Fungible Token Symbol** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS ft_symbol --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Fungible Token Decimals** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS ft_decimals --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Fungible Token Icon** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS ft_icon --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Fungible Token Metadata** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS ft_metadata --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Fungible Token Total Supply** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS ft_total_supply --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Add Authorized Caller** - State Changing Function Call

```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS add_authorized_caller --args '{"authorized_caller": "AUTHORIZED_CALLER_WALLET_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```

**Fungible Token Transfer** - State Changing Function Call

```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS ft_transfer --args '{"recipient_id": "RECEIVER_WALLET_ADDRESS", "amount": "FT_TO_BE_TRANSFERRED"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```

**Fungible Token Approve** - State Changing Function Call
This function approves the Spender to spend specified amount.
```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS ft_approve --args '{"spender_id": "SPENDER_WALLET_ADDRESS", "amount": "FT_APPROVED"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```

**Fungible Token Transfer From** - State Changing Function Call
This function is used by the approved Spender to transfer FT.
```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS ft_transfer_from --args '{"sender_id":  "OWNER_WALLET_ADDRESS", "recipient_id": "RECEIVER_WALLET_ADDRESS", "amount": "FT_TO_BE_TRANSFERRED"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```

**Fungible Token Allowance** - Read Only Function Call

```sh
l1x-cli-beta contract view INIT_CONTRACT_ADDRESS ft_allowance --args '{"owner_id": "OWNER_WALLET_ADDRESS", "spender_id": "SPENDER_WALLET_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

**Fungible Token Increase Allowance** - State Changing Function Call

```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS ft_increase_allowance –args '{"spender_id": " SPENDER_WALLET_ADDRESS ", "amount": "FT_INCREASE_BY"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```

**Fungible Token Decrease Allowance** - State Changing Function Call

```sh
l1x-cli-beta contract call INIT_CONTRACT_ADDRESS ft_decrease_allowance –args '{"spender_id": " SPENDER_WALLET_ADDRESS ", "amount": "FT_DECREASE_BY"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```




## License
This project is licensed under the MIT License. See the LICENSE file for more details.
