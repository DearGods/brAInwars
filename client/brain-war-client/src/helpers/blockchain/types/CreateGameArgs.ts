/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from "@metaplex-foundation/beet";
export type CreateGameArgs = {
  gameId: beet.bignum;
  waitForPlayersLimit: beet.bignum;
  hashedLimit: beet.bignum;
  entryFee: beet.bignum;
  timestamp: beet.bignum;
};

/**
 * @category userTypes
 * @category generated
 */
export const createGameArgsBeet = new beet.BeetArgsStruct<CreateGameArgs>(
  [
    ["gameId", beet.u64],
    ["waitForPlayersLimit", beet.u64],
    ["hashedLimit", beet.u64],
    ["entryFee", beet.u64],
    ["timestamp", beet.u64],
  ],
  "CreateGameArgs",
);
