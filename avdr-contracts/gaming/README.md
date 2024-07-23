# L1X Gaming Contract

This project implements gaming contracts using Rust.

<br>

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)

<br>

## Overview

This project contains smart contracts that implement gaming logic. It contains features such as:
- Administrator Control: Start and End the game.
- Lobby Management: Create and join lobby, retrieve lobby details.
- Player Notifications: Indicate readiness to start the game.
- Play Game: Store and manage game events.
- Scoreboard: View scores and user details within a lobby

<br>


## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust, Node JS, NVM and Cargo are installed. You can Set up Environment from [here](https://l1x-sdk.gitbook.io/l1x-developer-interface/v/interface-essentials/l1x-vm-sdk/l1x-native-sdk-for-l1x-vm/set-up-environment)

- Super, Emma, Ava and Bobby are four users having their own wallet with some non-zero balance.

- Super is deploying all the contracts


<br>

## Installation

All the two contracts need to be deployed and initialized in the given sequence.

```sh
mkdir gaming
cd gaming
```

<br>

### 1. Types Dependency



Initiate a new L1X project creation process with Cargo's L1X plugin.
 ```sh
cargo l1x create types
```

Goto src/lib.rs and paste the smart contract there. 

Goto src/score_board_interface.rs and paste the smart contract there.

Similarly, paste Cargo.toml file in the project.


These contracts serve as dependency for the Gaming contracts.


<br>

### 2. Game Installation


Initiate a new L1X project creation process with Cargo's L1X plugin.
 ```sh
cargo l1x create game
```

Goto src/lib.rs and paste the smart contract there. Similarly, paste Cargo.toml file in the project.


<br>


##### 2.1 Building the Game Contract
 ```sh
cd game
cargo l1x build
```
A CONTRACT_OBJECT_FILE **game.o** would be created as target/l1x/release/game.o

<br>


##### 2.2 Game Contract Deployment

Deploy the compiled L1X project to the L1X blockchain.

```sh
l1x-cli-beta contract deploy ./target/l1x/release/game.o --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```
You will get deployed contract address (GAME_DEPLOY_CONTRACT_ADDRESS) as the response of the above command. Use it to initialize your L1X project.

<br>


##### 2.3 Initialize the Game Contract

Initialize your deployed L1X project by setting up its base contract address.

```sh
l1x-cli-beta contract init GAME_DEPLOY_CONTRACT_ADDRESS --args '{"name": "GAME_NAME", "description": "GAME_DESCRIPTION", "game_id": "Game_ID"}'  --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```

On successful initialization of the project, you will get initialized contract address (GAME_INIT_CONTRACT_ADDRESS) as the response of the init command. Use it for further Readonly and State Changing Game Function calls


<br>



### 3. Score Board Installation


Initiate a new L1X project creation process with Cargo's L1X plugin.
 ```sh
cargo l1x create score_board
```

Goto src/lib.rs and paste the smart contract there.
Goto src/board.rs and paste the smart contract there.
Goto src/rules.rs and paste the smart contract there.


 Similarly, paste Cargo.toml file in the project.

<br>


##### 3.1 Building the Score Board Contract
 ```sh
cd score_board
cargo l1x build
```
A CONTRACT_OBJECT_FILE **score_board.o** would be created as target/l1x/release/score_board.o

<br>


##### 3.2 Score Board Contract Deployment

Deploy the compiled L1X project to the L1X blockchain.

```sh
l1x-cli-beta contract deploy ./target/l1x/release/score_board.o --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```
You will get deployed contract address (SCORE_BOARD_DEPLOY_CONTRACT_ADDRESS) as the response of the above command. Use it to initialize your L1X project.

<br>


##### 3.3 Initialize the Score Board Contract

Initialize your deployed L1X project by setting up its base contract address.

```sh
l1x-cli-beta contract init SCORE_BOARD_DEPLOY_CONTRACT_ADDRESS --args '{"name": "GAME_NAME", "description": "GAME_DESCRIPTION", "game_id": "Game_ID", "session_id": "SESSION_ID", "administrator":"GAME_INIT_CONTRACT_ADDRESS"}'  --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 100000
```

On successful initialization of the project, you will get initialized contract address (SCORE_BOARD_INIT_CONTRACT_ADDRESS) as the response of the init command. Use it for further Readonly and State Changing Score Board Function calls


<br>



## Usage

Here’s the sample scenario for Gaming contracts along with the sample functions and how to interact with the contract.


<br>



### Scenario

- Super is a game administrator. Super administrates the game server and the game contract.
- Ava creates the lobby.
- Emma and Bobby want to play with Ava so they join the lobby.
- Ava and Emma say that they are ready to start the game. But Bobby is not ready.
- Super starts the game. Bobby doesn't participate in the started game because he was not ready at that time when the game was started.
- Players store game events to score-board contract.
- Super ends the game.
- Anyone can check the leaderboard in score-board contract.

<br>


### Flow


**Create Lobby** - State Changing Function Call
- Step 1: Ava creates a new lobby. This command is run behalf of Ava.

```sh
l1x-cli-beta contract call GAME_INIT_CONTRACT_ADDRESS create_lobby --args '{"session_id": "SESSION_ID", "board": "SCORE_BOARD_INIT_CONTRACT_ADDRESS"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```


<br>


**Check Lobby Status** - Read Only Function Call
- Step 2: Ava checks lobby status. This command is run behalf of Ava
At this stage, _locked_ is false as the game has not yet started and players can join the lobby.
Also, Ava status _is_ready_ is false as Ava is not yet ready to start the game.

```sh
l1x-cli-beta contract view GAME_INIT_CONTRACT_ADDRESS get_lobby_by_session --args '{"session_id": "SESSION_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```


<br>


**Check Users** - Read Only Function Call
- Step 3: Check users connected to Score Board. The list should be empty. This command is run behalf of Super.


```sh
l1x-cli-beta contract view SCORE_BOARD_INIT_CONTRACT_ADDRESS users --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

<br>



**Join Lobby** - State Changing Function Call
- Step 4: Emma and Bobby join the lobby

<br>


- 4.1: Emma joins the lobby. This command is run behalf of Emma.

```sh
l1x-cli-beta contract call GAME_INIT_CONTRACT_ADDRESS join_lobby --args '{"session_id": "SESSION_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```

<br>


- 4.2: Bobby joins the lobby. This command is run behalf of Bobby.

```sh
l1x-cli-beta contract call GAME_INIT_CONTRACT_ADDRESS join_lobby --args '{"session_id": "SESSION_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```

<br>


**Ready to Start the Game** - State Changing Function Call
- Step 5: Ava and Emma report that they are ready to start the game.

<br>



- 5.1: Ava reports that she is ready to start the game. This command is run behalf of Ava.

```sh
l1x-cli-beta contract call GAME_INIT_CONTRACT_ADDRESS set_user_ready_status --args '{"session_id": "SESSION_ID", "status": true}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```

<br>


- 5.2: Emma reports that she is ready to start the game. This command is run behalf of Emma.

```sh
l1x-cli-beta contract call GAME_INIT_CONTRACT_ADDRESS set_user_ready_status --args '{"session_id": "SESSION_ID", "status": true}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```

<br>


**Check Lobby Status** - Read Only Function Call
- Step 6: Super checks lobby status. This command is run behalf of Super

```sh
l1x-cli-beta contract view GAME_INIT_CONTRACT_ADDRESS get_lobby_by_session --args '{"session_id": "SESSION_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

<br>



**Start Game** - State Changing Function Call

- Step 7: Super starts the game. This command is executed behalf of Super.


```sh
l1x-cli-beta contract call GAME_INIT_CONTRACT_ADDRESS start_game --args '{"session_id": "SESSION_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```

<br>



**Check Lobby Status** - Read Only Function Call
- Step 8: Super checks lobby status. Since the game has started, at this stage the score board status _locked_ should be true. Similarly, Ava and Emma status _is_ready_ is true whereas since Bobby is not ready to play, his _is_ready_ status is false. This command is run behalf of Super.

```sh
l1x-cli-beta contract view GAME_INIT_CONTRACT_ADDRESS get_lobby_by_session --args '{"session_id": "SESSION_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

<br>


**Check Users** - Read Only Function Call
- Step 9: Super checks the users connected to Score Board . The list should contain Emma's and Ava's addresses. This command is executed behalf of Super.


```sh
l1x-cli-beta contract view SCORE_BOARD_INIT_CONTRACT_ADDRESS users --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```


<br>


**Store Game Events** - State Changing Function Call
- Step 10: Ava and Emma store their game events


<br>


- 10.1: Ava stores the game event. This command is executed behalf of Ava.

- Note that the EVENT_DATA is to be passed on Base64 format.

```sh
echo -n "EVENT_DATA" | base64
```
You get BASE64_EVENT_DATA


```sh
l1x-cli-beta contract call SCORE_BOARD_INIT_CONTRACT_ADDRESS store_event --args '{"event": "BASE64_EVENT_DATA"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```

<br>


- 10.2: Emma stores the game event. This command is executed behalf of Emma.

- Note that the EVENT_DATA is to be passed on Base64 format.

```sh
echo -n "EVENT_DATA" | base64
```
You get BASE64_EVENT_DATA


```sh
l1x-cli-beta contract call SCORE_BOARD_INIT_CONTRACT_ADDRESS store_event --args '{"event": "BASE64_EVENT_DATA"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```

<br>


- 10.3: Ava stores the game event. This command is executed behalf of Ava.

- Note that the EVENT_DATA is to be passed on Base64 format.

```sh
echo -n "EVENT_DATA" | base64
```
You get BASE64_EVENT_DATA


```sh
l1x-cli-beta contract call SCORE_BOARD_INIT_CONTRACT_ADDRESS store_event --args '{"event": "BASE64_EVENT_DATA"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```


<br>


**Check Board** - Read Only Function Call
- Step 11: Super checks the Score Board . The list should contain Emma's and Ava's scores in the board, 1 and 2 respectively. This command is executed behalf of Super.


```sh
l1x-cli-beta contract view SCORE_BOARD_INIT_CONTRACT_ADDRESS board --args '{}' --endpoint https://v2-testnet-rpc.l1x.foundation
```

<br>


**End Game** - State Changing Function Call

- Step 12: Super ends the game. This command is executed behalf of Super.


```sh
l1x-cli-beta contract call GAME_INIT_CONTRACT_ADDRESS end_game --args '{"session_id": "SESSION_ID"}' --endpoint https://v2-testnet-rpc.l1x.foundation --fee_limit 1000000
```


<br>


## License
This project is licensed under the MIT License. See the LICENSE file for more details.
