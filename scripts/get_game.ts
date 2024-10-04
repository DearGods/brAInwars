import { Connection } from "@solana/web3.js";
import { processAndValidateTransaction } from "../tests/helpers";
import { setCrankInstruction } from "../tests/generated/wrappers";
import { getCrank, getGameAddress } from "../tests/generated/pdas";
import assert from "assert";
import { Game } from "../tests/generated";

enum Network {
  DEVNET = "DEVNET",
  MAINNET = "MAINNET",
}

function getConnection(network: Network): Connection {
  if (network === Network.DEVNET) {
    return new Connection(
      "https://devnet.helius-rpc.com/?api-key=e4bbdd51-a7f2-4353-92e0-f85b1b4c9200",
    );
  } else {
    return new Connection(
      "https://mainnet.helius-rpc.com/?api-key=e4bbdd51-a7f2-4353-92e0-f85b1b4c9200",
    );
  }
}

async function main() {
  const connection = getConnection(Network.DEVNET);

  const game_ids = [255834605, 16538850];
  for (const game_id of game_ids) {
    const [gameAddress] = getGameAddress(game_id);
    const game = await Game.fromAccountAddress(connection, gameAddress);
    console.log("Game", game_id, game);
  }
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
