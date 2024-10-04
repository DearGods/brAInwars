import { GamePlayersStatus } from '@/helpers/apiTypes'

export default interface IGame {
  id: string;
  entry_fee: number;
  mint: string;
  created_at: string;
  game_status: string;
  num_participants: string;
  winner: string | null;
  reveled_limit: string | null;
  player_status: string | null;
  start_time: number;
  current_time: string;
  players_statuses: Array<GamePlayersStatus>;
}