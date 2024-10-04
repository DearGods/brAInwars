export type DepositModalType =
  | "MISSING_MONEY"
  | "ADDING_MONEY"
  | "CLOSED"
  | "WITHDRAW";

export enum DepositModal {
  MISSING_MONEY = "MISSING_MONEY",
  ADDING_MONEY = "ADDING_MONEY",
  WITHDRAW = "WITHDRAW",
  CLOSED = "CLOSED",
}
