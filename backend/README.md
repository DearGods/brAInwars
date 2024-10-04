# Games

## Solana Dice

## Brain Wars

### Description

Players place a bet at the start of the game.
On the server side, a random number is generated that determines
the length of the game, 0`[s]` to 45`[s]`.

Each player decides when to cash out, and that exited last, wins.
If no player exited and the game ended, all players lose.

Players can buy a tip that gives them a range of the current game length.
Higher tier tips cost more and give more accurate ranges.

For example, a tip will say `this game will last between 10 and 20 seconds`.

### Rules

1. Players pay a fee to play.
2. Players decide when to exit a round.
3. Last player to exit wins.
4. If no player exits, all players lose.

### Implementation

#### Server

- `axum` for the web server
- `websockets` for quick communication
- `postgres` for storing game data
- `react` for the frontend

#### Blockchain

Each round we'll sample `solana` blockchain and get the latest block hash.
We'll convert this blockhash to a number between 0 and 45.
We'll then use this number to determine the length of the game.
We'll encrypt this data and store it on the blockchain, when the game ends we'll publish the decryption key.

## References

1. [axum - confused about the websocket/chat example model](https://github.com/tokio-rs/axum/discussions/604)
2. [axum - websocket example](https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/main.rs)
3. [axum - SQLx](https://github.com/tokio-rs/axum/blob/main/examples/sqlx-postgres/src/main.rs)
4. [axum - Zero2Prod](https://github.com/mattiapenati/zero2prod)
5. [axum - examples](https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md)
6. [Code Coverage - Rust](https://rrmprogramming.com/article/code-coverage-in-rust/)
7. [axum-login](https://github.com/maxcountryman/axum-login/blob/main/examples/simple-with-role/src/main.rs)
8. [TS-Rust Binding](https://github.com/Aleph-Alpha/ts-rs)
9. [Rust websockets messaging](https://blog.logrocket.com/build-websocket-server-with-rust/)
10.
