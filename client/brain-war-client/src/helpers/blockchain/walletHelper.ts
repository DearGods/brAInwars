import {
  BlockheightBasedTransactionConfirmationStrategy,
  Connection,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { Wallet } from "@/helpers/walletContext";
import { TxnResult } from "@/helpers/blockchain/helpers";

export async function processTransactionWithWallet(
  instructions: TransactionInstruction[],
  connection: Connection,
  wallet: Wallet,
): Promise<TxnResult> {
  const tx = new Transaction();
  instructions.map((i) => tx.add(i));
  const blockStats = await connection.getLatestBlockhash();
  tx.recentBlockhash = blockStats.blockhash;
  tx.feePayer = wallet?.publicKey();
  const signedTx = await wallet.signTransaction(tx);
  const sig = await connection.sendRawTransaction(signedTx.serialize(), {
    maxRetries: 3,
    preflightCommitment: "confirmed",
    skipPreflight: true,
  });
  const strategy: BlockheightBasedTransactionConfirmationStrategy = {
    signature: sig,
    blockhash: blockStats.blockhash,
    lastValidBlockHeight: blockStats.lastValidBlockHeight,
  };
  const result = await connection.confirmTransaction(strategy, "processed");
  return {
    Signature: sig,
    SignatureResult: result.value,
  };
}
