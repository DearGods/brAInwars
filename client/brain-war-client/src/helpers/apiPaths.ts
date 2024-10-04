enum ApiPaths {
  // Auth
  LOGOUT = "/auth/logout",
  CREATE_BALANCE = "/balance/create_user_balance",
  GET_BALANCE = "/balance/get_user_balance",
  DEPOSIT = "/balance/deposit",
  WITHDRAW = "/balance/withdraw",
  ME = "/users/get",
  UPDATE_USER_NAME = "/users/update_name",
  JOIN_GAME = "/games/join_game",
  LEAVE_GAME = "/games/leave_game",
  JOINED_LEFT_BEFORE_START = "/games/leave_game_only_you_here",
  // Public
  LOGIN = "/auth/login",
  NONCE = "/auth/nonce",
  GET_GAME = "/games/get",
  UNAUTH_GET_GAME = "/games/unauth_get",
  GET_GAMES = "/games/index",
  CREATE_USER = "/users/create",
}

export default ApiPaths;
