/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from "@metaplex-foundation/beet";
import * as web3 from "@solana/web3.js";
import * as beetSolana from "@metaplex-foundation/beet-solana";
import { PlayerAction, playerActionBeet } from "../types/PlayerAction";

/**
 * Arguments used to create {@link PlayersActions}
 * @category Accounts
 * @category generated
 */
export type PlayersActionsArgs = {
  bump: number;
  currentSize: beet.bignum;
  game: web3.PublicKey;
  playersActions: PlayerAction[];
  players: web3.PublicKey[];
};

export const playersActionsDiscriminator = [84, 199, 73, 122, 101, 22, 148, 97];
/**
 * Holds the data for the {@link PlayersActions} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class PlayersActions implements PlayersActionsArgs {
  private constructor(
    readonly bump: number,
    readonly currentSize: beet.bignum,
    readonly game: web3.PublicKey,
    readonly playersActions: PlayerAction[],
    readonly players: web3.PublicKey[],
  ) {}

  /**
   * Creates a {@link PlayersActions} instance from the provided args.
   */
  static fromArgs(args: PlayersActionsArgs) {
    return new PlayersActions(
      args.bump,
      args.currentSize,
      args.game,
      args.playersActions,
      args.players,
    );
  }

  /**
   * Deserializes the {@link PlayersActions} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0,
  ): [PlayersActions, number] {
    return PlayersActions.deserialize(accountInfo.data, offset);
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link PlayersActions} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
    commitmentOrConfig?: web3.Commitment | web3.GetAccountInfoConfig,
  ): Promise<PlayersActions> {
    const accountInfo = await connection.getAccountInfo(
      address,
      commitmentOrConfig,
    );
    if (accountInfo == null) {
      throw new Error(`Unable to find PlayersActions account at ${address}`);
    }
    return PlayersActions.fromAccountInfo(accountInfo, 0)[0];
  }

  /**
   * Provides a {@link web3.Connection.getProgramAccounts} config builder,
   * to fetch accounts matching filters that can be specified via that builder.
   *
   * @param programId - the program that owns the accounts we are filtering
   */
  static gpaBuilder(
    programId: web3.PublicKey = new web3.PublicKey(
      "2yKYJeX7NDF6gSHANJqnxHuAuHuBx5Qf3s9oAmMpWzxh",
    ),
  ) {
    return beetSolana.GpaBuilder.fromStruct(programId, playersActionsBeet);
  }

  /**
   * Deserializes the {@link PlayersActions} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [PlayersActions, number] {
    return playersActionsBeet.deserialize(buf, offset);
  }

  /**
   * Serializes the {@link PlayersActions} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return playersActionsBeet.serialize({
      accountDiscriminator: playersActionsDiscriminator,
      ...this,
    });
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link PlayersActions} for the provided args.
   *
   * @param args need to be provided since the byte size for this account
   * depends on them
   */
  static byteSize(args: PlayersActionsArgs) {
    const instance = PlayersActions.fromArgs(args);
    return playersActionsBeet.toFixedFromValue({
      accountDiscriminator: playersActionsDiscriminator,
      ...instance,
    }).byteSize;
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link PlayersActions} data from rent
   *
   * @param args need to be provided since the byte size for this account
   * depends on them
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    args: PlayersActionsArgs,
    connection: web3.Connection,
    commitment?: web3.Commitment,
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(
      PlayersActions.byteSize(args),
      commitment,
    );
  }

  /**
   * Returns a readable version of {@link PlayersActions} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      bump: this.bump,
      currentSize: (() => {
        const x = <{ toNumber: () => number }>this.currentSize;
        if (typeof x.toNumber === "function") {
          try {
            return x.toNumber();
          } catch (_) {
            return x;
          }
        }
        return x;
      })(),
      game: this.game.toBase58(),
      playersActions: this.playersActions,
      players: this.players,
    };
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const playersActionsBeet = new beet.FixableBeetStruct<
  PlayersActions,
  PlayersActionsArgs & {
    accountDiscriminator: number[] /* size: 8 */;
  }
>(
  [
    ["accountDiscriminator", beet.uniformFixedSizeArray(beet.u8, 8)],
    ["bump", beet.u8],
    ["currentSize", beet.u64],
    ["game", beetSolana.publicKey],
    ["playersActions", beet.array(playerActionBeet)],
    ["players", beet.array(beetSolana.publicKey)],
  ],
  PlayersActions.fromArgs,
  "PlayersActions",
);
