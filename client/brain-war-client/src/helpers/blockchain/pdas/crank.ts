import { Connection, PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { Crank, PROGRAM_ID } from "../index";

export function getCrankAddress(): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(anchor.utils.bytes.utf8.encode("CRANK"))],
    PROGRAM_ID,
  );
}

export function getCrankTokenAddress(mint: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(anchor.utils.bytes.utf8.encode("CRANK")), mint.toBuffer()],
    PROGRAM_ID,
  );
}

export async function getCrank(connection: Connection): Promise<Crank> {
  const [crankAddress] = getCrankAddress();
  return await Crank.fromAccountAddress(connection, crankAddress, "confirmed");
}
