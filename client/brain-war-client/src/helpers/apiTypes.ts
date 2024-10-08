/*
 Generated by typeshare 1.9.2
*/

export type Date = string;

export type Id = string;

export enum PLAYER_STATUS {
  NOT_IN_GAME = "NOT_IN_GAME",
  JOINED_GAME = "JOINED_GAME",
  LEFT_GAME = "LEFT_GAME",
}

export interface GamePlayersStatus {
  wallet_address: string;
  name: string;
  created_at: Date;
  player_status: PLAYER_STATUS;
}

export interface LoginRequest {
  wallet_address: string;
  signed_message: string;
  nonce: string;
}

export enum ResponseStatus {
  Success = "Success",
  Failure = "Failure",
}

export interface LoginResponse {
  uuid?: Id;
  wallet_address?: string;
  name?: string;
  status: ResponseStatus;
  message?: string;
}

export interface LogoutRequest {
  wallet_address: string;
}

export interface LogoutResponse {
  status: ResponseStatus;
  message?: string;
}

export interface GetGameRequest {
  game_id: Id;
}

export enum GameStatus {
  WaitingForPlayers = "WaitingForPlayers",
  OnGoing = "OnGoing",
  Finished = "Finished",
  Settled = "Settled",
}

export interface GetGameResponse {
  id: Id;
  game_id: number;
  entry_fee: number;
  mint: string;
  created_at: Date;
  game_status: GameStatus;
  num_participants: number;
  winner?: string;
  reveled_limit?: number;
  player_status?: PLAYER_STATUS;
  start_time?: number;
  counter_end_time?: number;
  current_time: Date;
  players_statuses: GamePlayersStatus[];
  status: ResponseStatus;
  message?: string;
}

export interface IndexGameResponse {
  games: GetGameResponse[];
  status: ResponseStatus;
  message?: string;
}

export interface JoinGameRequest {
  game_id: Id;
}

export interface JoinGameResponse {
  game_id?: Id;
  user_id?: Id;
  status: ResponseStatus;
  message?: string;
}

export interface LeaveGameRequest {
  game_id: Id;
}

export interface LeaveGameResponse {
  game_id?: Id;
  user_id?: Id;
  status: ResponseStatus;
  message?: string;
}

export interface UnAuthGetGameRequest {
  game_id: Id;
}

export interface UnAuthGetGameResponse {
  id: Id;
  created_at: Date;
  game_id: number;
  entry_fee: number;
  mint: string;
  game_status: GameStatus;
  num_participants: number;
  winner?: string;
  reveled_limit?: number;
  counter_end_time?: number;
  player_status?: PLAYER_STATUS;
  start_time?: number;
  current_time: Date;
  players_statuses: GamePlayersStatus[];
  status: ResponseStatus;
  message?: string;
}

export interface CreateUserRequest {
  wallet_address: string;
}

export interface CreateUserResponse {
  id?: Id;
  wallet_address?: string;
  nonce?: string;
  status: ResponseStatus;
  message?: string;
}

export interface GetUserRequest {
  wallet_address: string;
}

export interface GetUserResponse {
  id?: Id;
  wallet_address: string;
  name: string;
  status: ResponseStatus;
  message?: string;
}

export interface UpdateUserNameRequest {
  wallet_address: string;
  name: string;
}

export interface UpdateUserNameResponse {
  id?: Id;
  wallet_address: string;
  name: string;
  status: ResponseStatus;
  message?: string;
}

export enum MsgName {
  GAME_COUNTER = "GAME_COUNTER",
  NEW_GAME_CREATED = "NEW_GAME_CREATED",
  GAME_SETTLED = "GAME_SETTLED",
  GAME_ENDED = "GAME_ENDED",
  GAME_STARTED = "GAME_STARTED",
  PLAYER_JOINED = "PLAYER_JOINED",
  PLAYER_LEFT = "PLAYER_LEFT",
}

export interface PlayerJoined {
  msg_name: MsgName;
  game_id: Id;
  wallet_address: string;
  name: string;
  msg_id: Id;
}

export interface PlayerLeft {
  msg_name: MsgName;
  game_id: Id;
  wallet_address: string;
  name: string;
  msg_id: Id;
}

export interface GameStarted {
  msg_name: MsgName;
  game_id: Id;
  msg_id: Id;
}

export interface NewGameCreated {
  msg_name: MsgName;
  game_id: Id;
  msg_id: Id;
}

export interface GameEnded {
  msg_name: MsgName;
  game_id: Id;
  msg_id: Id;
  winner?: string;
}

export interface GameSettled {
  msg_name: MsgName;
  game_id: Id;
  msg_id: Id;
}

export interface GameCounter {
  msg_name: MsgName;
  game_id: Id;
  msg_id: Id;
  start_time_from_now_in_milliseconds: number;
}
