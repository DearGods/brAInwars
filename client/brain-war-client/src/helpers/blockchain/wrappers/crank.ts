import { Connection, PublicKey, TransactionInstruction } from "@solana/web3.js";
import {
  createInitCrankInstruction,
  createSetCrankInstruction,
  InitCrankInstructionAccounts,
  SetCrankInstructionAccounts,
  SetCrankInstructionArgs,
} from "../instructions";
import { getCrankAddress } from "../pdas/crank";
import { accountExists } from "../helpers";

export type SetCrankInstructionInputs = {
  fee: number;
  signer: PublicKey;
  crankSigner: PublicKey;
  connection: Connection;
};

export async function setCrankInstruction({
  fee,
  signer,
  crankSigner,
  connection,
}: SetCrankInstructionInputs): Promise<TransactionInstruction[]> {
  const [crank] = getCrankAddress();
  const exists = await accountExists(connection, crank);
  const instructions: TransactionInstruction[] = [];
  if (exists === false) {
    const initAccounts: InitCrankInstructionAccounts = {
      signer,
      crankSigner,
      crank,
    };
    instructions.push(createInitCrankInstruction(initAccounts));
  }
  const args: SetCrankInstructionArgs = {
    args: {
      fee,
    },
  };
  const accounts: SetCrankInstructionAccounts = {
    signer,
    crankSigner,
    crank,
  };
  instructions.push(createSetCrankInstruction(accounts, args));
  return instructions;
}
