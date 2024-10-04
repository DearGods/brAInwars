import { Connection, PublicKey } from "@solana/web3.js";
import { getAccount, Account } from "@solana/spl-token";

export async function getTokenAccount(
  connection: Connection,
  address: PublicKey,
): Promise<Account> {
  return await getAccount(connection, address, "confirmed");
}
