import { GameStatus, GamePlayersStatus } from "../apiTypes";
export default interface IGameState {
  id: string | null;
  roomId: number;
  winnerMode: boolean;
  start_time: Date | null;
  counter_time: number | null;
  live: boolean;
  timer: boolean;
  buttonShaking: boolean;
  aboutToStart: boolean;
  playersStatus: GamePlayersStatus[] | null;
  status: GameStatus | null;
  winner: string | null;
}
