import { Connection, PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { Game, PROGRAM_ID } from "../index";
import BN from "bn.js";

export function getGameAddress(id: number): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from(anchor.utils.bytes.utf8.encode("GAME")),
      new BN(id).toBuffer("le", 8),
    ],
    PROGRAM_ID,
  );
}

export async function getGame(
  connection: Connection,
  id: number,
): Promise<Game> {
  const [game] = getGameAddress(id);
  return await Game.fromAccountAddress(connection, game, "confirmed");
}

export function getGameTokenAddress(
  id: number,
  mint: PublicKey,
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from(anchor.utils.bytes.utf8.encode("GAME")),
      mint.toBuffer(),
      new BN(id).toBuffer("le", 8),
    ],
    PROGRAM_ID,
  );
}
