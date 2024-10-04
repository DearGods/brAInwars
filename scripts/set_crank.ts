import { Connection } from "@solana/web3.js";
import { processAndValidateTransaction } from "../tests/helpers";
import { setCrankInstruction } from "../tests/generated/wrappers";
import { getCrank } from "../tests/generated/pdas";
import assert from "assert";
import { loadWalletKey } from "../tests/wallet-loader";

enum Network {
  DEVNET = "DEVNET",
  MAINNET = "MAINNET",
}

function getConnection(network: Network): Connection {
  if (network === Network.DEVNET) {
    return new Connection("https://api.devnet.solana.com");
  } else {
    return new Connection("https://api.mainnet-beta.solana.com");
  }
}

async function main() {
  const crankWallet = loadWalletKey("./backend/fixtures/crank.json");
  const connection = getConnection(Network.DEVNET);

  const instructions = await setCrankInstruction({
    fee: 500,
    signer: crankWallet.publicKey,
    crankSigner: crankWallet.publicKey,
    connection,
  });

  await processAndValidateTransaction(instructions, connection, crankWallet);

  const crankAccount = await getCrank(connection);

  assert.equal(crankAccount.fee, 500);
  assert.equal(
    crankAccount.signer.toBase58(),
    crankWallet.publicKey.toBase58(),
  );
}

main()
  .then(() => {
    console.log("Done");
    process.exit(0);
  })
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
