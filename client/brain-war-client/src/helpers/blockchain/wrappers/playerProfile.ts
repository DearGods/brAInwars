import { Connection, PublicKey, TransactionInstruction } from "@solana/web3.js";
import {
  createInitPlayerProfileInstruction,
  createInitPlayerProfileTokenAccountInstruction,
  createInitSignerTokenAccountInstruction,
  createPlayerProfileDepositInstruction,
  createPlayerProfileWithdrawInstruction,
  InitPlayerProfileInstructionAccounts,
  InitPlayerProfileTokenAccountInstructionAccounts,
  InitSignerTokenAccountInstructionAccounts,
  PlayerProfileDepositInstructionAccounts,
  PlayerProfileDepositInstructionArgs,
  PlayerProfileWithdrawInstructionAccounts,
  PlayerProfileWithdrawInstructionArgs,
} from "../instructions";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import {
  getPlayerProfileAddress,
  getPlayerProfileTokenAddress,
} from "../pdas/playerProfile";
import { accountExists } from "../helpers";
import web3 from "@solana/web3.js";

export type PlayerProfileDepositInstructionInputs = {
  amount: number;
  signer: PublicKey;
  mint: PublicKey;
  connection: Connection;
};

export async function playerProfileDepositInstruction({
  amount,
  signer,
  mint,
  connection,
}: PlayerProfileDepositInstructionInputs): Promise<TransactionInstruction[]> {
  const instructions: TransactionInstruction[] = [];

  const args: PlayerProfileDepositInstructionArgs = {
    args: {
      amount,
    },
  };

  const [playerProfile] = getPlayerProfileAddress(signer);
  const [playerTokenAccount] = getPlayerProfileTokenAddress(signer, mint);
  const signerTokenAccount = getAssociatedTokenAddressSync(mint, signer);

  const exists1 = await accountExists(connection, signerTokenAccount);
  const exists2 = await accountExists(connection, playerProfile);
  const exists3 = await accountExists(connection, playerTokenAccount);

  if (exists1 === false) {
    const initSignerTokenAccountAccounts: InitSignerTokenAccountInstructionAccounts =
      {
        signer,
        signerTokenAccount,
        mint,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      };
    instructions.push(
      createInitSignerTokenAccountInstruction(initSignerTokenAccountAccounts),
    );
  }

  if (exists2 === false) {
    const initPlayerProfileAccounts: InitPlayerProfileInstructionAccounts = {
      signer,
      playerProfile,
      mint,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    };
    instructions.push(
      createInitPlayerProfileInstruction(initPlayerProfileAccounts),
    );
  }
  if (exists3 === false) {
    const initPlayerTokenAccountAccounts: InitPlayerProfileTokenAccountInstructionAccounts =
      {
        signer,
        playerProfile,
        playerTokenAccount,
        mint,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      };
    instructions.push(
      createInitPlayerProfileTokenAccountInstruction(
        initPlayerTokenAccountAccounts,
      ),
    );
  }

  const accounts: PlayerProfileDepositInstructionAccounts = {
    signer,
    signerTokenAccount,
    playerProfile,
    playerTokenAccount,
    mint,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  };

  instructions.push(createPlayerProfileDepositInstruction(accounts, args));
  return instructions;
}

export type PlayerProfileWithdrawInstructionInputs = {
  amount: number;
  signer: PublicKey;
  mint: PublicKey;
};

export function playerProfileWithdrawInstruction({
  signer,
  amount,
  mint,
}: PlayerProfileWithdrawInstructionInputs): TransactionInstruction {
  const args: PlayerProfileWithdrawInstructionArgs = {
    args: {
      amount,
    },
  };
  const [playerProfile] = getPlayerProfileAddress(signer);
  const [playerTokenAccount] = getPlayerProfileTokenAddress(signer, mint);
  const signerTokenAccount = getAssociatedTokenAddressSync(mint, signer);

  const accounts: PlayerProfileWithdrawInstructionAccounts = {
    signer,
    signerTokenAccount,
    playerProfile,
    playerTokenAccount,
    mint,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  };

  return createPlayerProfileWithdrawInstruction(accounts, args);
}
