import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BrainWars } from "../target/types/brain_wars";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import assert from "assert";
import {
  airdrop,
  generateHashInput,
  getOrCreateTokenAccountInstruction,
  processTransaction,
} from "./helpers";
import {
  createMint,
  getAssociatedTokenAddress,
  mintTo,
} from "@solana/spl-token";
import { getCrankAddress, getGameAddress } from "./generated/pdas";

export const crankSigner = Keypair.generate();
export const randomSigner = Keypair.generate();
export const admin = Keypair.generate();
export const players = [
  Keypair.generate(),
  Keypair.generate(),
  Keypair.generate(),
];
export const tokenMintAuthority = Keypair.generate();
export let mint: PublicKey;
export let token9Decimals: PublicKey;

describe("0-prep", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BrainWars as Program<BrainWars>;

  it("Program ID", async () => {
    const [crank] = getCrankAddress();
    console.log(`program.id: ${program.programId.toBase58()}`);
    console.log("crank:", crank.toBase58());
    console.log("hash:", generateHashInput(crank, 1));
    console.log("hash:", generateHashInput(crank, 2));
  });

  it("Airdrops", async () => {
    for (const key of [...players, crankSigner, randomSigner, admin]) {
      await airdrop(program, key.publicKey, LAMPORTS_PER_SOL * 50_000);
    }
  });

  it("Create main mint", async () => {
    mint = await createMint(
      program.provider.connection,
      admin,
      tokenMintAuthority.publicKey,
      tokenMintAuthority.publicKey,
      9,
    );
  });

  it("Mint tokens", async () => {
    for (const key of [...players, crankSigner, randomSigner, admin]) {
      const instructions = await getOrCreateTokenAccountInstruction(
        mint,
        key.publicKey,
        program.provider.connection,
      );
      if (instructions === null) {
        continue;
      }
      const sig = await processTransaction(
        [instructions],
        program.provider.connection,
        key,
      );
      const txn = await program.provider.connection.getParsedTransaction(
        sig.Signature,
        "confirmed",
      );
      assert.equal(
        sig.SignatureResult.err,
        null,
        `${mint.toBase58()}\n${txn?.meta?.logMessages.join("\n")}`,
      );

      await mintTo(
        program.provider.connection,
        admin,
        mint,
        await getAssociatedTokenAddress(mint, key.publicKey),
        tokenMintAuthority,
        LAMPORTS_PER_SOL * 50_000,
      );
    }
  });
});
