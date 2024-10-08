/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from "@metaplex-foundation/beet";
import * as web3 from "@solana/web3.js";
import { SetCrankArgs, setCrankArgsBeet } from "../types/SetCrankArgs";

/**
 * @category Instructions
 * @category SetCrank
 * @category generated
 */
export type SetCrankInstructionArgs = {
  args: SetCrankArgs;
};
/**
 * @category Instructions
 * @category SetCrank
 * @category generated
 */
export const setCrankStruct = new beet.BeetArgsStruct<
  SetCrankInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */;
  }
>(
  [
    ["instructionDiscriminator", beet.uniformFixedSizeArray(beet.u8, 8)],
    ["args", setCrankArgsBeet],
  ],
  "SetCrankInstructionArgs",
);
/**
 * Accounts required by the _setCrank_ instruction
 *
 * @property [_writable_, **signer**] signer
 * @property [] crankSigner
 * @property [_writable_] crank
 * @category Instructions
 * @category SetCrank
 * @category generated
 */
export type SetCrankInstructionAccounts = {
  signer: web3.PublicKey;
  crankSigner: web3.PublicKey;
  crank: web3.PublicKey;
  systemProgram?: web3.PublicKey;
  rent?: web3.PublicKey;
  anchorRemainingAccounts?: web3.AccountMeta[];
};

export const setCrankInstructionDiscriminator = [
  225, 180, 235, 15, 254, 126, 108, 115,
];

/**
 * Creates a _SetCrank_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category SetCrank
 * @category generated
 */
export function createSetCrankInstruction(
  accounts: SetCrankInstructionAccounts,
  args: SetCrankInstructionArgs,
  programId = new web3.PublicKey(
    "2yKYJeX7NDF6gSHANJqnxHuAuHuBx5Qf3s9oAmMpWzxh",
  ),
) {
  const [data] = setCrankStruct.serialize({
    instructionDiscriminator: setCrankInstructionDiscriminator,
    ...args,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.signer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.crankSigner,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.crank,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.rent ?? web3.SYSVAR_RENT_PUBKEY,
      isWritable: false,
      isSigner: false,
    },
  ];

  if (accounts.anchorRemainingAccounts != null) {
    for (const acc of accounts.anchorRemainingAccounts) {
      keys.push(acc);
    }
  }

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  });
  return ix;
}
