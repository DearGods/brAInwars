import { Connection, PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { PlayerProfile, PROGRAM_ID } from "../index";
import { Account } from "@solana/spl-token";
import { getTokenAccount } from "./token";

export function getPlayerProfileAddress(
  player: PublicKey,
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from(anchor.utils.bytes.utf8.encode("PLAYER_PROFILE")),
      player.toBuffer(),
    ],
    PROGRAM_ID,
  );
}

export async function getPlayerProfile(
  connection: Connection,
  player: PublicKey,
): Promise<PlayerProfile> {
  const [playerProfileAddress] = getPlayerProfileAddress(player);
  return await PlayerProfile.fromAccountAddress(
    connection,
    playerProfileAddress,
  );
}

export function getPlayerProfileTokenAddress(
  player: PublicKey,
  mint: PublicKey,
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from(anchor.utils.bytes.utf8.encode("PLAYER_PROFILE")),
      player.toBuffer(),
      mint.toBuffer(),
    ],
    PROGRAM_ID,
  );
}

export async function getPlayerTokenAccount(
  connection: Connection,
  player: PublicKey,
  mint: PublicKey,
): Promise<Account> {
  const [playerTokenAddress] = getPlayerProfileTokenAddress(player, mint);
  return getTokenAccount(connection, playerTokenAddress);
}
