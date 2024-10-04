import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BrainWars } from "../target/types/brain_wars";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import assert from "assert";
import { processAndValidateTransaction, sleep } from "./helpers";
import { NATIVE_MINT } from "@solana/spl-token";
import {
  playerProfileDepositInstruction,
  playerProfileWithdrawInstruction,
} from "./generated/wrappers";
import { mint, players } from "./0-prep";
import {
  getPlayerProfile,
  getPlayerProfileAddress,
  getPlayerProfileTokenAddress,
} from "./generated/pdas/playerProfile";

export const fee = 100;

describe("2-player-profile", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BrainWars as Program<BrainWars>;

  it("Players Deposit", async () => {
    for (const player of players) {
      const instructions = await playerProfileDepositInstruction({
        amount: 200 * LAMPORTS_PER_SOL,
        signer: player.publicKey,
        mint: NATIVE_MINT,
        connection: program.provider.connection,
      });

      await processAndValidateTransaction(
        instructions,
        program.provider.connection,
        player,
      );

      const [playerProfileAddress] = getPlayerProfileAddress(player.publicKey);
      const playerProfile = await getPlayerProfile(
        program.provider.connection,
        player.publicKey,
      );
      assert.equal(
        playerProfile.player.toBase58(),
        player.publicKey.toBase58(),
      );
      const balance =
        await program.provider.connection.getBalance(playerProfileAddress);
      assert.ok(balance >= 200 * LAMPORTS_PER_SOL);
    }

    for (const player of players) {
      const instructions = await playerProfileDepositInstruction({
        amount: 200 * LAMPORTS_PER_SOL,
        signer: player.publicKey,
        mint: mint,
        connection: program.provider.connection,
      });

      await processAndValidateTransaction(
        instructions,
        program.provider.connection,
        player,
      );

      const playerProfile = await getPlayerProfile(
        program.provider.connection,
        player.publicKey,
      );
      assert.equal(
        playerProfile.player.toBase58(),
        player.publicKey.toBase58(),
      );

      const [playerProfileTokenAddress] = getPlayerProfileTokenAddress(
        player.publicKey,
        mint,
      );
      const balance = await program.provider.connection.getTokenAccountBalance(
        playerProfileTokenAddress,
      );
      assert.equal(balance.value.amount, 200 * LAMPORTS_PER_SOL);
    }
  });

  it("Players Withdraw", async () => {
    for (const player of players) {
      const playerProfile = await getPlayerProfile(
        program.provider.connection,
        player.publicKey,
      );
      assert.equal(
        playerProfile.player.toBase58(),
        player.publicKey.toBase58(),
      );
      const [playerProfileAddress] = getPlayerProfileAddress(player.publicKey);
      const balancePlayerProfilePre =
        await program.provider.connection.getBalance(playerProfileAddress);
      assert.ok(balancePlayerProfilePre >= 200 * LAMPORTS_PER_SOL);

      const balancePre = await program.provider.connection.getBalance(
        player.publicKey,
        "confirmed",
      );
      const instruction = playerProfileWithdrawInstruction({
        amount: 3 * LAMPORTS_PER_SOL,
        signer: player.publicKey,
        mint: NATIVE_MINT,
      });

      await processAndValidateTransaction(
        [instruction],
        program.provider.connection,
        player,
      );

      await sleep(1000);

      const balancePost = await program.provider.connection.getBalance(
        player.publicKey,
        "confirmed",
      );

      const diff = balancePost - balancePre;
      assert.ok(diff >= 2.99 * LAMPORTS_PER_SOL);
    }

    for (const player of players) {
      const [playerProfileTokenAddress] = getPlayerProfileTokenAddress(
        player.publicKey,
        mint,
      );
      const balancePre =
        await program.provider.connection.getTokenAccountBalance(
          playerProfileTokenAddress,
        );
      const instruction = playerProfileWithdrawInstruction({
        amount: 2 * LAMPORTS_PER_SOL,
        signer: player.publicKey,
        mint: mint,
      });

      await processAndValidateTransaction(
        [instruction],
        program.provider.connection,
        player,
      );
      await sleep(1000);

      const playerProfile = await getPlayerProfile(
        program.provider.connection,
        player.publicKey,
      );
      assert.equal(
        playerProfile.player.toBase58(),
        player.publicKey.toBase58(),
      );

      const balancePost =
        await program.provider.connection.getTokenAccountBalance(
          playerProfileTokenAddress,
        );

      const diff = balancePre.value.uiAmount - balancePost.value.uiAmount;
      assert.equal(diff, 2);
    }
  });
});
