import { Connection, PublicKey, TransactionInstruction } from "@solana/web3.js";
import {
  createCreateGameInstruction,
  createFinishGameInstruction,
  CreateGameInstructionAccounts,
  CreateGameInstructionArgs,
  createInitCrankTokenAccountInstruction,
  createJoinGameInstruction,
  createLeaveGameInstruction,
  createSettleGameInstruction,
  createStartGameInstruction,
  FinishGameInstructionAccounts,
  FinishGameInstructionArgs,
  InitCrankTokenAccountInstructionAccounts,
  JoinGameInstructionAccounts,
  JoinGameInstructionArgs,
  LeaveGameInstructionAccounts,
  LeaveGameInstructionArgs,
  SettleGameInstructionAccounts,
  SettleGameInstructionArgs,
  StartGameInstructionAccounts,
  StartGameInstructionArgs,
} from "../instructions";
import { getCrankAddress, getCrankTokenAddress } from "../pdas";
import { getGame, getGameAddress, getGameTokenAddress } from "../pdas";
import { getPlayersActionsAddress } from "../pdas/playersActions";
import { ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { getPlayerProfileAddress, getPlayerProfileTokenAddress } from "../pdas";
import { accountExists } from "../../helpers";

export type CreateGameInstructionInputs = {
  signer: PublicKey;
  gameId: number;
  waitForPlayersLimit: number;
  hashedLimit: number;
  entryFee: number;
  mint: PublicKey;
  timestamp: number;
};

export function createGameInstruction({
  signer,
  gameId,
  waitForPlayersLimit,
  hashedLimit,
  entryFee,
  mint,
  timestamp,
}: CreateGameInstructionInputs): TransactionInstruction {
  const args: CreateGameInstructionArgs = {
    args: {
      gameId,
      waitForPlayersLimit,
      hashedLimit,
      entryFee,
      timestamp,
    },
  };

  const [crank] = getCrankAddress();
  const [game] = getGameAddress(gameId);
  const [gameTokenAccount] = getGameTokenAddress(gameId, mint);
  const [playersActions] = getPlayersActionsAddress(gameId);

  const accounts: CreateGameInstructionAccounts = {
    signer,
    crank,
    game,
    gameTokenAccount,
    playersActions,
    mint,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  };

  return createCreateGameInstruction(accounts, args);
}

export type JoinGameInstructionInputs = {
  signer: PublicKey;
  player: PublicKey;
  gameId: number;
  connection: Connection;
  timestamp: number;
};

export async function joinGameInstruction({
  connection,
  signer,
  gameId,
  player,
  timestamp,
}: JoinGameInstructionInputs): Promise<TransactionInstruction[]> {
  const instructions: TransactionInstruction[] = [];

  const game = await getGame(connection, gameId);
  const mint = game.mint;

  const [gameAddress] = getGameAddress(gameId);
  const [crank] = getCrankAddress();
  const [crankTokenAccount] = getCrankTokenAddress(mint);
  const [gameTokenAccount] = getGameTokenAddress(gameId, mint);
  const [playersActions] = getPlayersActionsAddress(gameId);

  const [playerProfile] = getPlayerProfileAddress(player);
  const [playerTokenAccount] = getPlayerProfileTokenAddress(player, mint);

  const exists = await accountExists(connection, crankTokenAccount);
  if (exists === false) {
    const existAccounts: InitCrankTokenAccountInstructionAccounts = {
      signer,
      crank,
      crankTokenAccount,
      mint,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    };
    instructions.push(createInitCrankTokenAccountInstruction(existAccounts));
  }

  const args: JoinGameInstructionArgs = {
    args: {
      timestamp,
    },
  };

  const accounts: JoinGameInstructionAccounts = {
    signer,
    crank,
    crankTokenAccount,
    player,
    playerProfile,
    playerTokenAccount,
    game: gameAddress,
    gameTokenAccount,
    playersActions,
    mint,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  };

  instructions.push(createJoinGameInstruction(accounts, args));
  return instructions;
}

export type LeaveGameInstructionInputs = {
  connection: Connection;
  signer: PublicKey;
  gameId: number;
  player: PublicKey;
  timestamp: number;
};

export async function leaveGameInstruction({
  connection,
  signer,
  gameId,
  player,
  timestamp,
}: LeaveGameInstructionInputs): Promise<TransactionInstruction> {
  const game = await getGame(connection, gameId);
  const mint = game.mint;

  const [gameAddress] = getGameAddress(gameId);
  const [crank] = getCrankAddress();
  const [gameTokenAccount] = getGameTokenAddress(gameId, mint);
  const [playersActions] = getPlayersActionsAddress(gameId);

  const [playerProfile] = getPlayerProfileAddress(player);
  const [playerTokenAccount] = getPlayerProfileTokenAddress(player, mint);

  const args: LeaveGameInstructionArgs = {
    args: {
      timestamp,
    },
  };

  const accounts: LeaveGameInstructionAccounts = {
    signer,
    crank,
    player,
    playerProfile,
    playerTokenAccount,
    game: gameAddress,
    gameTokenAccount,
    playersActions,
    mint,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  };

  return createLeaveGameInstruction(accounts, args);
}

export type FinishGameInstructionInputs = {
  reveledLimit: number;
  reveledSalt: number;
  signer: PublicKey;
  winner: PublicKey;
  gameId: number;
  timestamp: number;
};

export function finishGameInstruction({
  signer,
  gameId,
  reveledLimit,
  reveledSalt,
  timestamp,
  winner,
}: FinishGameInstructionInputs): TransactionInstruction {
  const args: FinishGameInstructionArgs = {
    args: {
      reveledLimit,
      reveledSalt,
      timestamp,
    },
  };

  const [crank] = getCrankAddress();
  const [game] = getGameAddress(gameId);
  const [playersActions] = getPlayersActionsAddress(gameId);

  const accounts: FinishGameInstructionAccounts = {
    winner,
    signer,
    crank,
    game,
    playersActions,
  };

  return createFinishGameInstruction(accounts, args);
}

export type SettleGameInstructionInputs = {
  signer: PublicKey;
  gameId: number;
  connection: Connection;
  timestamp: number;
};

export async function settleGameInstruction({
  signer,
  gameId,
  connection,
  timestamp,
}: SettleGameInstructionInputs): Promise<TransactionInstruction> {
  const game = await getGame(connection, gameId);
  const [gameAddress] = getGameAddress(gameId);
  const [crank] = getCrankAddress();
  const { winner, mint } = game;
  const [gameTokenAccount] = getGameTokenAddress(gameId, mint);
  const [playersActions] = getPlayersActionsAddress(gameId);
  const [playerProfile] = getPlayerProfileAddress(winner!);
  const [playerTokenAccount] = getPlayerProfileTokenAddress(winner!, mint);

  const args: SettleGameInstructionArgs = {
    args: {
      timestamp,
    },
  };

  const accounts: SettleGameInstructionAccounts = {
    signer,
    crank,
    winner: winner!,
    game: gameAddress,
    playerProfile,
    playerTokenAccount,
    gameTokenAccount,
    playersActions,
    mint,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  };

  return createSettleGameInstruction(accounts, args);
}

export type StartGameInstructionInputs = {
  signer: PublicKey;
  gameId: number;
  timestamp: number;
};

export function startGameInstruction({
  signer,
  gameId,
  timestamp,
}: StartGameInstructionInputs): TransactionInstruction {
  const [crank] = getCrankAddress();
  const [game] = getGameAddress(gameId);

  const args: StartGameInstructionArgs = {
    args: {
      timestamp,
    },
  };

  const accounts: StartGameInstructionAccounts = {
    signer,
    crank,
    game,
  };

  return createStartGameInstruction(accounts, args);
}
