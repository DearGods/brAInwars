/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from "@solana/web3.js";
import * as beet from "@metaplex-foundation/beet";
import * as beetSolana from "@metaplex-foundation/beet-solana";

/**
 * Arguments used to create {@link PlayerProfile}
 * @category Accounts
 * @category generated
 */
export type PlayerProfileArgs = {
  bump: number;
  player: web3.PublicKey;
  gamesPlayed: beet.bignum;
  gamesWon: beet.bignum;
};

export const playerProfileDiscriminator = [82, 226, 99, 87, 164, 130, 181, 80];
/**
 * Holds the data for the {@link PlayerProfile} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class PlayerProfile implements PlayerProfileArgs {
  private constructor(
    readonly bump: number,
    readonly player: web3.PublicKey,
    readonly gamesPlayed: beet.bignum,
    readonly gamesWon: beet.bignum,
  ) {}

  /**
   * Creates a {@link PlayerProfile} instance from the provided args.
   */
  static fromArgs(args: PlayerProfileArgs) {
    return new PlayerProfile(
      args.bump,
      args.player,
      args.gamesPlayed,
      args.gamesWon,
    );
  }

  /**
   * Deserializes the {@link PlayerProfile} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0,
  ): [PlayerProfile, number] {
    return PlayerProfile.deserialize(accountInfo.data, offset);
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link PlayerProfile} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
    commitmentOrConfig?: web3.Commitment | web3.GetAccountInfoConfig,
  ): Promise<PlayerProfile> {
    const accountInfo = await connection.getAccountInfo(
      address,
      commitmentOrConfig,
    );
    if (accountInfo == null) {
      throw new Error(`Unable to find PlayerProfile account at ${address}`);
    }
    return PlayerProfile.fromAccountInfo(accountInfo, 0)[0];
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
    return beetSolana.GpaBuilder.fromStruct(programId, playerProfileBeet);
  }

  /**
   * Deserializes the {@link PlayerProfile} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [PlayerProfile, number] {
    return playerProfileBeet.deserialize(buf, offset);
  }

  /**
   * Serializes the {@link PlayerProfile} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return playerProfileBeet.serialize({
      accountDiscriminator: playerProfileDiscriminator,
      ...this,
    });
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link PlayerProfile}
   */
  static get byteSize() {
    return playerProfileBeet.byteSize;
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link PlayerProfile} data from rent
   *
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    connection: web3.Connection,
    commitment?: web3.Commitment,
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(
      PlayerProfile.byteSize,
      commitment,
    );
  }

  /**
   * Determines if the provided {@link Buffer} has the correct byte size to
   * hold {@link PlayerProfile} data.
   */
  static hasCorrectByteSize(buf: Buffer, offset = 0) {
    return buf.byteLength - offset === PlayerProfile.byteSize;
  }

  /**
   * Returns a readable version of {@link PlayerProfile} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      bump: this.bump,
      player: this.player.toBase58(),
      gamesPlayed: (() => {
        const x = <{ toNumber: () => number }>this.gamesPlayed;
        if (typeof x.toNumber === "function") {
          try {
            return x.toNumber();
          } catch (_) {
            return x;
          }
        }
        return x;
      })(),
      gamesWon: (() => {
        const x = <{ toNumber: () => number }>this.gamesWon;
        if (typeof x.toNumber === "function") {
          try {
            return x.toNumber();
          } catch (_) {
            return x;
          }
        }
        return x;
      })(),
    };
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const playerProfileBeet = new beet.BeetStruct<
  PlayerProfile,
  PlayerProfileArgs & {
    accountDiscriminator: number[] /* size: 8 */;
  }
>(
  [
    ["accountDiscriminator", beet.uniformFixedSizeArray(beet.u8, 8)],
    ["bump", beet.u8],
    ["player", beetSolana.publicKey],
    ["gamesPlayed", beet.u64],
    ["gamesWon", beet.u64],
  ],
  PlayerProfile.fromArgs,
  "PlayerProfile",
);
