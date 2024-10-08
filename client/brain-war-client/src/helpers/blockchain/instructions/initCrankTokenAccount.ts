/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as splToken from "@solana/spl-token";
import * as beet from "@metaplex-foundation/beet";
import * as web3 from "@solana/web3.js";

/**
 * @category Instructions
 * @category InitCrankTokenAccount
 * @category generated
 */
export const initCrankTokenAccountStruct = new beet.BeetArgsStruct<{
  instructionDiscriminator: number[] /* size: 8 */;
}>(
  [["instructionDiscriminator", beet.uniformFixedSizeArray(beet.u8, 8)]],
  "InitCrankTokenAccountInstructionArgs",
);
/**
 * Accounts required by the _initCrankTokenAccount_ instruction
 *
 * @property [_writable_, **signer**] signer
 * @property [_writable_] crank
 * @property [_writable_] crankTokenAccount
 * @property [] mint
 * @property [] associatedTokenProgram
 * @category Instructions
 * @category InitCrankTokenAccount
 * @category generated
 */
export type InitCrankTokenAccountInstructionAccounts = {
  signer: web3.PublicKey;
  crank: web3.PublicKey;
  crankTokenAccount: web3.PublicKey;
  mint: web3.PublicKey;
  systemProgram?: web3.PublicKey;
  tokenProgram?: web3.PublicKey;
  associatedTokenProgram: web3.PublicKey;
  rent?: web3.PublicKey;
  anchorRemainingAccounts?: web3.AccountMeta[];
};

export const initCrankTokenAccountInstructionDiscriminator = [
  209, 35, 78, 195, 31, 11, 18, 163,
];

/**
 * Creates a _InitCrankTokenAccount_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @category Instructions
 * @category InitCrankTokenAccount
 * @category generated
 */
export function createInitCrankTokenAccountInstruction(
  accounts: InitCrankTokenAccountInstructionAccounts,
  programId = new web3.PublicKey(
    "2yKYJeX7NDF6gSHANJqnxHuAuHuBx5Qf3s9oAmMpWzxh",
  ),
) {
  const [data] = initCrankTokenAccountStruct.serialize({
    instructionDiscriminator: initCrankTokenAccountInstructionDiscriminator,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.signer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.crank,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.crankTokenAccount,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.mint,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.tokenProgram ?? splToken.TOKEN_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.associatedTokenProgram,
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
