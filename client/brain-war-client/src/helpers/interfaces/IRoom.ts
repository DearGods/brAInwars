import IGame from './IGame';
import IMessage from './IMessage';
import IPlayerActionRecord from "@/helpers/interfaces/IPlayerActionRecord";

export default interface IRoom {
  id: number;
  entry_fee: number;
  currentGameId: string;
  messages: IMessage[]; 
  games: IGame[];
  playersActions: IPlayerActionRecord[];
  watchingPlayers: number;
}