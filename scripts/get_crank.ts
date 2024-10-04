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
  const crank = await getCrank(connection);
  console.log("crank", {
    crank: {
      signer: crank.signer.toBase58(),
    },
    crankWallet: crankWallet.publicKey.toBase58(),
  });
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
