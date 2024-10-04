type action = "JOINED" | "BAILED" | "GAME STARTED" | "GAME ENDED" | "WON";

export default interface IPlayerActionRecord {
  id: number;
  name: string;
  action: action;
  created_at: Date;
}
