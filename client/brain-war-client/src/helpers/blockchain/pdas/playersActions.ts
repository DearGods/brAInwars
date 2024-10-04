import { PlayersActions, PROGRAM_ID } from "../index";
import { Connection, PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";

export function getPlayersActionsAddress(gameId: number): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from(anchor.utils.bytes.utf8.encode("PLAYERS_ACTIONS")),
      new BN(gameId).toBuffer("le", 8),
    ],
    PROGRAM_ID,
  );
}

export async function getPlayersActions(
  connection: Connection,
  gameId: number,
): Promise<PlayersActions> {
  const [playersActions] = getPlayersActionsAddress(gameId);
  return await PlayersActions.fromAccountAddress(
    connection,
    playersActions,
    "confirmed",
  );
}
