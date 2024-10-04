import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BrainWars } from "../target/types/brain_wars";
import assert from "assert";
import { processAndValidateTransaction, processTransaction } from "./helpers";
import { getCrank, getCrankAddress } from "./generated/pdas/crank";
import { setCrankInstruction } from "./generated/wrappers";
import { admin, crankSigner, randomSigner } from "./0-prep";

export const fee = 100;

describe("1-crank", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BrainWars as Program<BrainWars>;

  it("Set Crank - First time - Admin signs", async () => {
    const instructions = await setCrankInstruction({
      fee,
      signer: admin.publicKey,
      crankSigner: admin.publicKey,
      connection: program.provider.connection,
    });

    await processAndValidateTransaction(
      instructions,
      program.provider.connection,
      admin,
    );

    const crankAccount = await getCrank(program.provider.connection);

    assert.equal(crankAccount.fee, fee);
    assert.equal(crankAccount.signer.toBase58(), admin.publicKey.toBase58());

    const instructions2 = await setCrankInstruction({
      fee,
      signer: admin.publicKey,
      crankSigner: crankSigner.publicKey,
      connection: program.provider.connection,
    });

    await processAndValidateTransaction(
      instructions2,
      program.provider.connection,
      admin,
    );

    const crankAccount2 = await getCrank(program.provider.connection);

    assert.equal(crankAccount.fee, fee);
    assert.equal(
      crankAccount2.signer.toBase58(),
      crankSigner.publicKey.toBase58(),
    );
  });

  it("Set Crank - After init - Admin signs", async () => {
    const instructions1 = await setCrankInstruction({
      fee,
      signer: crankSigner.publicKey,
      crankSigner: randomSigner.publicKey,
      connection: program.provider.connection,
    });

    await processAndValidateTransaction(
      instructions1,
      program.provider.connection,
      crankSigner,
    );

    const crankAccount1 = await getCrank(program.provider.connection);

    assert.equal(crankAccount1.fee, fee);
    assert.equal(
      crankAccount1.signer.toBase58(),
      randomSigner.publicKey.toBase58(),
    );

    const instructions2 = await setCrankInstruction({
      fee,
      signer: randomSigner.publicKey,
      crankSigner: crankSigner.publicKey,
      connection: program.provider.connection,
    });

    await processAndValidateTransaction(
      instructions2,
      program.provider.connection,
      randomSigner,
    );

    const crankAccount2 = await getCrank(program.provider.connection);

    assert.equal(crankAccount2.fee, fee);
    assert.equal(
      crankAccount2.signer.toBase58(),
      crankSigner.publicKey.toBase58(),
    );
  });

  it("Set Crank - Second time - Random signs", async () => {
    const [crank] = getCrankAddress();

    const instructions = await setCrankInstruction({
      fee,
      signer: randomSigner.publicKey,
      crankSigner: randomSigner.publicKey,
      connection: program.provider.connection,
    });

    const txn = await processTransaction(
      instructions,
      program.provider.connection,
      randomSigner,
    );

    assert.ok(txn.SignatureResult.err !== null);
    const crankAccount = await getCrank(program.provider.connection);

    assert.equal(crankAccount.fee, fee);
    assert.equal(
      crankAccount.signer.toBase58(),
      crankSigner.publicKey.toBase58(),
    );
  });
});
