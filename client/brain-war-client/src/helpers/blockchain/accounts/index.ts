export * from "./Crank";
export * from "./Game";
export * from "./PlayerProfile";
export * from "./PlayersActions";

import { Crank } from "./Crank";
import { Game } from "./Game";
import { PlayerProfile } from "./PlayerProfile";
import { PlayersActions } from "./PlayersActions";

export const accountProviders = { Crank, Game, PlayerProfile, PlayersActions };
