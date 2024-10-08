/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as splToken from "@solana/spl-token";
import * as beet from "@metaplex-foundation/beet";
import * as web3 from "@solana/web3.js";
import {
  LeaveGameGameArgs,
  leaveGameGameArgsBeet,
} from "../types/LeaveGameGameArgs";

/**
 * @category Instructions
 * @category LeaveGame
 * @category generated
 */
export type LeaveGameInstructionArgs = {
  args: LeaveGameGameArgs;
};
/**
 * @category Instructions
 * @category LeaveGame
 * @category generated
 */
export const leaveGameStruct = new beet.BeetArgsStruct<
  LeaveGameInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */;
  }
>(
  [
    ["instructionDiscriminator", beet.uniformFixedSizeArray(beet.u8, 8)],
    ["args", leaveGameGameArgsBeet],
  ],
  "LeaveGameInstructionArgs",
);
/**
 * Accounts required by the _leaveGame_ instruction
 *
 * @property [_writable_, **signer**] signer
 * @property [] crank
 * @property [] player
 * @property [_writable_] playerProfile
 * @property [_writable_] playerTokenAccount
 * @property [_writable_] game
 * @property [_writable_] gameTokenAccount
 * @property [_writable_] playersActions
 * @property [] mint
 * @property [] associatedTokenProgram
 * @category Instructions
 * @category LeaveGame
 * @category generated
 */
export type LeaveGameInstructionAccounts = {
  signer: web3.PublicKey;
  crank: web3.PublicKey;
  player: web3.PublicKey;
  playerProfile: web3.PublicKey;
  playerTokenAccount: web3.PublicKey;
  game: web3.PublicKey;
  gameTokenAccount: web3.PublicKey;
  playersActions: web3.PublicKey;
  mint: web3.PublicKey;
  systemProgram?: web3.PublicKey;
  tokenProgram?: web3.PublicKey;
  associatedTokenProgram: web3.PublicKey;
  rent?: web3.PublicKey;
  anchorRemainingAccounts?: web3.AccountMeta[];
};

export const leaveGameInstructionDiscriminator = [
  218, 226, 6, 0, 243, 34, 125, 201,
];

/**
 * Creates a _LeaveGame_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category LeaveGame
 * @category generated
 */
export function createLeaveGameInstruction(
  accounts: LeaveGameInstructionAccounts,
  args: LeaveGameInstructionArgs,
  programId = new web3.PublicKey(
    "2yKYJeX7NDF6gSHANJqnxHuAuHuBx5Qf3s9oAmMpWzxh",
  ),
) {
  const [data] = leaveGameStruct.serialize({
    instructionDiscriminator: leaveGameInstructionDiscriminator,
    ...args,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.signer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.crank,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.player,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.playerProfile,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.playerTokenAccount,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.game,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.gameTokenAccount,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.playersActions,
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
