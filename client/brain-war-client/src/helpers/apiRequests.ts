import axios from "axios";
import { Result } from "@badrap/result";
import {
  CreateUserRequest,
  CreateUserResponse,
  GetGameRequest,
  GetGameResponse,
  GetUserRequest,
  GetUserResponse,
  IndexGameResponse,
  JoinGameRequest,
  JoinGameResponse,
  LeaveGameRequest,
  UpdateUserNameRequest,
  LeaveGameResponse,
  LoginRequest,
  LoginResponse,
  LogoutRequest,
  LogoutResponse,
  UnAuthGetGameRequest,
  UnAuthGetGameResponse,
} from "./apiTypes";
import ApiPaths from "./apiPaths";

const baseOrigin =
  (window.location.host.includes("localhost")
    ? "http://localhost:8000"
    : window.location.origin) + "/api";

export async function login(
  props: LoginRequest,
): Promise<Result<LoginResponse, Error>> {
  try {
    const response = await axios
      .post(baseOrigin + ApiPaths.LOGIN, props, { withCredentials: true })
      .then((res: { data: any }) => res.data);
    return Result.ok(response);
  } catch (error: any) {
    return Result.err(error);
  }
}

export async function logout(
  props: LogoutRequest,
): Promise<Result<LogoutResponse, Error>> {
  try {
    const response = await axios
      .post(baseOrigin + ApiPaths.LOGOUT, props, { withCredentials: true })
      .then((res: { data: any }) => res.data);
    return Result.ok(response);
  } catch (error: any) {
    return Result.err(error);
  }
}

export async function register(
  props: CreateUserRequest,
): Promise<Result<CreateUserResponse, Error>> {
  try {
    const response = await axios
      .post(baseOrigin + ApiPaths.CREATE_USER, props, { withCredentials: true })
      .then((res: { data: any }) => res.data);
    return Result.ok(response);
  } catch (error: any) {
    return Result.err(error);
  }
}

export async function join_game(
  props: JoinGameRequest,
): Promise<Result<JoinGameResponse, Error>> {
  try {
    const response = await axios
      .post(baseOrigin + ApiPaths.JOIN_GAME, props, { withCredentials: true })
      .then((res: { data: any }) => res.data);
    return Result.ok(response);
  } catch (error: any) {
    return Result.err(error);
  }
}

export async function leave_game(
  props: LeaveGameRequest,
): Promise<Result<LeaveGameResponse, Error>> {
  try {
    const response = await axios
      .post(baseOrigin + ApiPaths.LEAVE_GAME, props, { withCredentials: true })
      .then((res: { data: any }) => res.data);
    return Result.ok(response);
  } catch (error: any) {
    return Result.err(error);
  }
}

export async function update_user_name(
  props: UpdateUserNameRequest,
): Promise<Result<LeaveGameResponse, Error>> {
  try {
    const response = await axios
      .post(baseOrigin + ApiPaths.UPDATE_USER_NAME, props, {
        withCredentials: true,
      })
      .then((res: { data: any }) => res.data);
    return Result.ok(response);
  } catch (error: any) {
    return Result.err(error);
  }
}

export async function unauth_get_game(
  props: UnAuthGetGameRequest,
): Promise<Result<UnAuthGetGameResponse, Error>> {
  try {
    const response = await axios
      .get(
        baseOrigin + ApiPaths.UNAUTH_GET_GAME + `?game_id=${props.game_id}`,
        { withCredentials: true },
      )
      .then((res: { data: any }) => res.data);
    return Result.ok(response);
  } catch (error: any) {
    return Result.err(error);
  }
}

export async function get_game(
  props: GetGameRequest,
): Promise<Result<GetGameResponse, Error>> {
  try {
    const response = await axios
      .get(baseOrigin + ApiPaths.GET_GAME + `?game_id=${props.game_id}`, {
        withCredentials: true,
      })
      .then((res: { data: any }) => res.data);
    return Result.ok(response);
  } catch (error: any) {
    return Result.err(error);
  }
}

export async function get_games(): Promise<Result<IndexGameResponse, Error>> {
  try {
    const response = await axios
      .get(baseOrigin + ApiPaths.GET_GAMES, { withCredentials: true })
      .then((res: { data: any }) => res.data);
    return Result.ok(response);
  } catch (error: any) {
    return Result.err(error);
  }
}

export async function get_user(
  props: GetUserRequest,
): Promise<Result<GetUserResponse, Error>> {
  try {
    const response = await axios
      .post(baseOrigin + ApiPaths.ME, props, { withCredentials: true })
      .then((res: { data: any }) => res.data);
    return Result.ok(response);
  } catch (error: any) {
    return Result.err(error);
  }
}
