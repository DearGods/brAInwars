import * as anchor from "@coral-xyz/anchor";
import assert from "assert";
import { Program } from "@coral-xyz/anchor";
import { BrainWars } from "../target/types/brain_wars";
import {
  createGameInstruction,
  finishGameInstruction,
  joinGameInstruction,
  leaveGameInstruction,
  settleGameInstruction,
  startGameInstruction,
} from "./generated/wrappers";
import { crankSigner, players } from "./0-prep";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { NATIVE_MINT } from "@solana/spl-token";
import {
  generateHashInput,
  Hash,
  processAndValidateTransaction,
  processTransaction,
} from "./helpers";
import {
  getGame,
  getGameAddress,
  getPlayerProfileAddress,
} from "./generated/pdas";
import { getPlayersActions } from "./generated/pdas/playersActions";

export const fee = 100;

const gameId = 2;
const waitForPlayersLimit = 1 * 1000;
let hash: Hash;
let gameLimit = 10 * 1000;

describe("4-game - Locks check", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BrainWars as Program<BrainWars>;

  it("Create Game", async () => {
    const [gameAddress] = getGameAddress(gameId);
    hash = generateHashInput(gameAddress, gameLimit);
    const instruction = createGameInstruction({
      signer: crankSigner.publicKey,
      gameId,
      waitForPlayersLimit,
      hashedLimit: hash.hash,
      entryFee: LAMPORTS_PER_SOL,
      mint: NATIVE_MINT,
      timestamp: Date.now(),
    });

    await processAndValidateTransaction(
      [instruction],
      program.provider.connection,
      crankSigner,
    );
    const game = await getGame(program.provider.connection, gameId);

    assert.equal(game.gameId, gameId);
    assert.equal(game.waitForPlayersLimit, waitForPlayersLimit);
    assert.equal(game.hashedLimit, hash.hash);
    assert.equal(game.entryFee, LAMPORTS_PER_SOL);
    assert.equal(game.mint.toBase58(), NATIVE_MINT.toBase58());
    assert.equal(game.gameStatus, 0);
    assert.equal(game.reveledLimit, null);
    assert.equal(game.reveledSalt, null);
    assert.equal(game.winner, null);

    const playersActions = await getPlayersActions(
      program.provider.connection,
      gameId,
    );
    assert.equal(playersActions.players, 0);
    assert.equal(playersActions.game.toBase58(), gameAddress.toBase58());
    assert.equal(playersActions.playersActions.length, 0);
  });

  it("First player joins", async () => {
    const [playerProfileAddress] = getPlayerProfileAddress(
      players[0].publicKey,
    );
    const balancePre =
      await program.provider.connection.getBalance(playerProfileAddress);
    const instructions = await joinGameInstruction({
      signer: crankSigner.publicKey,
      player: players[0].publicKey,
      gameId,
      connection: program.provider.connection,
      timestamp: Date.now(),
    });

    await processAndValidateTransaction(
      instructions,
      program.provider.connection,
      crankSigner,
    );
    const game = await getGame(program.provider.connection, gameId);
    const [gameAddress] = getGameAddress(gameId);

    assert.equal(game.gameStatus, 0);

    const playersActions = await getPlayersActions(
      program.provider.connection,
      gameId,
    );

    assert.equal(playersActions.players.length, 1);
    assert.equal(playersActions.playersActions.length, 1);
    const balancePost =
      await program.provider.connection.getBalance(playerProfileAddress);
    assert.equal(
      balancePre - balancePost,
      LAMPORTS_PER_SOL * (1 + fee / 10_000),
    );
  });

  it("Second player joins", async () => {
    const instructions = await joinGameInstruction({
      signer: crankSigner.publicKey,
      player: players[1].publicKey,
      gameId,
      connection: program.provider.connection,
      timestamp: Date.now(),
    });

    await processAndValidateTransaction(
      instructions,
      program.provider.connection,
      crankSigner,
    );
    const game = await getGame(program.provider.connection, gameId);

    assert.equal(game.gameStatus, 0);

    const playersActions = await getPlayersActions(
      program.provider.connection,
      gameId,
    );

    assert.equal(playersActions.players.length, 2);
    assert.equal(playersActions.playersActions.length, 2);
  });

  it("Start Game", async () => {
    const instruction = startGameInstruction({
      signer: crankSigner.publicKey,
      gameId,
      timestamp: Date.now(),
    });

    await processAndValidateTransaction(
      [instruction],
      program.provider.connection,
      crankSigner,
    );
  });

  it("Third player joins", async () => {
    const instructions = await joinGameInstruction({
      signer: crankSigner.publicKey,
      player: players[2].publicKey,
      gameId,
      connection: program.provider.connection,
      timestamp: Date.now(),
    });

    const sig = await processTransaction(
      instructions,
      program.provider.connection,
      crankSigner,
    );
    const txn = await program.provider.connection.getParsedTransaction(
      sig.Signature,
      "confirmed",
    );

    assert.ok(
      sig.SignatureResult.err !== null,
      `${txn?.meta?.logMessages.join("\n")}\n\n${JSON.stringify(sig)}`,
    );

    const game = await getGame(program.provider.connection, gameId);

    assert.equal(game.gameStatus, 1);

    const playersActions = await getPlayersActions(
      program.provider.connection,
      gameId,
    );

    assert.equal(playersActions.players.length, 2);
    assert.equal(playersActions.playersActions.length, 2);
  });

  it("First player leaves", async () => {
    const instruction = await leaveGameInstruction({
      connection: program.provider.connection,
      signer: crankSigner.publicKey,
      player: players[0].publicKey,
      gameId,
      timestamp: Date.now(),
    });

    await processAndValidateTransaction(
      [instruction],
      program.provider.connection,
      crankSigner,
    );

    const game = await getGame(program.provider.connection, gameId);

    assert.equal(game.gameStatus, 1);

    const playersActions = await getPlayersActions(
      program.provider.connection,
      gameId,
    );

    assert.equal(playersActions.players.length, 2);
    assert.equal(playersActions.playersActions.length, 3);
  });

  it("Finish game", async () => {
    const instruction = finishGameInstruction({
      signer: crankSigner.publicKey,
      gameId,
      reveledLimit: gameLimit,
      reveledSalt: hash.salt,
      timestamp: Date.now(),
    });

    await processAndValidateTransaction(
      [instruction],
      program.provider.connection,
      crankSigner,
    );

    const game = await getGame(program.provider.connection, gameId);
    assert.equal(game.gameStatus, 2);
    assert.equal(game.winner.toBase58(), players[0].publicKey.toBase58());
    assert.equal(game.reveledLimit, gameLimit);
  });

  it("Settle game", async () => {
    const [playerProfileAddress] = getPlayerProfileAddress(
      players[0].publicKey,
    );
    const balancePre =
      await program.provider.connection.getBalance(playerProfileAddress);

    const instruction = await settleGameInstruction({
      signer: crankSigner.publicKey,
      gameId,
      connection: program.provider.connection,
      timestamp: Date.now(),
    });
    await processAndValidateTransaction(
      [instruction],
      program.provider.connection,
      crankSigner,
    );

    const game = await getGame(program.provider.connection, gameId);
    assert.equal(game.gameStatus, 3);
    const balancePost =
      await program.provider.connection.getBalance(playerProfileAddress);

    assert.equal(balancePost - balancePre, 2 * LAMPORTS_PER_SOL);
  });
});
