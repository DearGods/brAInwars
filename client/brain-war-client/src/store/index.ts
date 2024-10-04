import { createStore } from "vuex";
import IPlayerActionRecord from "@/helpers/interfaces/IPlayerActionRecord";
import IGameState from "@/helpers/interfaces/IGameState";
import IPlayerState from "@/helpers/interfaces/IPlayerState";
import INotification from "@/helpers/interfaces/INotification";
import IChatState from "@/helpers/interfaces/IChatState";
import IRoom from "@/helpers/interfaces/IRoom";
import IMenuState from "@/helpers/interfaces/IMenuState";
import IMessage from "@/helpers/interfaces/IMessage";
import { Wallet } from "@/helpers/walletContext";
import * as ApiRequests from "@/helpers/apiRequests";
import bs58 from "bs58";
import { Connection } from "@solana/web3.js";
import {
  playerProfileDepositInstruction,
  playerProfileWithdrawInstruction,
} from "@/helpers/blockchain/wrappers";
import { NATIVE_MINT } from "@solana/spl-token";
import { processTransactionWithWallet } from "@/helpers/blockchain/walletHelper";
import { getWalletBalance, sleep } from "@/helpers/blockchain/helpers";
import { logger } from "@/helpers/logger";
import {
  GameStatus,
  GamePlayersStatus,
  PLAYER_STATUS,
  GetGameResponse,
  MsgName,
} from "@/helpers/apiTypes";
import IGame from "@/helpers/interfaces/IGame";
import {
  PlayerActionMsgEnum,
  PlayerActionMsgValues,
} from "@/helpers/enums/PlayerActionMsgEnum";
import { DepositModal, DepositModalType } from "@/helpers/feTypes";

const baseWsOrigin =
  (window.location.host.includes("localhost")
    ? "ws://localhost:8000"
    : window.location.origin.replace("http", "ws")) + "/api";

const REQUIRES_DEPOSIT_MESSAGES = [
  "Player Balance Not Found, player need to deposit",
  "Player Balance Too Low, player need to deposit",
];

export default createStore({
  state: {
    // auth_chat_ws_notifier: null as WebSocket | null,
    game_ws_notifier: null as WebSocket | null,
    unauth_chat_ws_notifier: null as WebSocket | null,
    balance: null as number | null,
    connection: null as null | Connection,
    wallet: null as Wallet | null,
    wallet_address: null as string | null,
    user_id: null as string | null,
    nonce: null as string | null,
    signedMessage: null as string | null,
    loggedIn: false,
    showDepositModal: DepositModal.CLOSED as DepositModalType,
    gameAudioMode: true as boolean,
    gameMusicMode: true as boolean,
    game: {
      id: null,
      status: null,
      roomId: -1,
      winnerMode: false,
      start_time: null,
      counter_time: null,
      live: false,
      buttonShaking: false,
      timer: false,
      aboutToStart: false,
      playersStatus: null,
    } as IGameState,
    roomId: 0 as number,
    rooms: [] as IRoom[],

    notification: {} as INotification,
    playingAudioFile: null as string | null,
    player: {
      name: "Guest",
      playing: false,
      bailedAt: null,
    } as IPlayerState,

    chat: {
      text: "",
      mode: false,
      messages: null,
    } as IChatState,

    menu: {
      isOpen: false,
    } as IMenuState,
  },

  getters: {
    // getAuthChatWSNotifier(state) {
    //   return state.auth_chat_ws_notifier;
    // },

    getPlayingAudioFile(state) {
      return state.playingAudioFile;
    },

    getGameWSNotifier(state) {
      return state.game_ws_notifier;
    },

    getUnAuthChatWSNotifier(state) {
      return state.unauth_chat_ws_notifier;
    },

    getUserBalance(state) {
      return state.balance ? (state.balance / 1000000000).toFixed(2) : ""; // 1,000,000,000;
    },

    getDepositModalMode(state): DepositModalType {
      return state.showDepositModal;
    },

    getShowDepositModal(state): string {
      return state.showDepositModal;
    },

    // getConnection(state) {
    //   return state.connection;
    // },

    getSignedMessage(state) {
      return state.signedMessage;
    },

    isLoggedIn(state) {
      return state.loggedIn;
    },

    isGameButtonShaking(state): boolean {
      return state.game.buttonShaking;
    },

    getUserId(state) {
      return state.user_id;
    },

    getWalletAddress(state) {
      return state.wallet_address;
    },

    getGameId(state) {
      return state.game.id;
    },

    getGameStartTime(state) {
      return state.game.start_time;
    },

    getGameCounter(state) {
      return state.game.counter_time;
    },

    getCurrentRoom(state) {
      return state.rooms.find((room) => room.id === state.game.roomId);
    },

    getGamePrice(state) {
      return state.rooms.find((room) => room.id === state.game.roomId)
        ?.entry_fee;
    },

    getNonce(state) {
      return state.nonce;
    },

    getWallet(state) {
      return state.wallet;
    },

    isGameLive(state) {
      return state.game.status === GameStatus.OnGoing;
    },

    isGamePending(state) {
      return state.game.status === GameStatus.WaitingForPlayers;
    },

    isGameFinished(state) {
      return state.game.status === GameStatus.Finished;
    },

    isTimerRunning(state) {
      return state.game.timer;
    },

    getRooms(state) {
      return state.rooms;
    },

    isGameAboutToStart(state) {
      return state.game.aboutToStart;
    },

    isPlaying(state) {
      return state.player.playing;
    },

    getPlayerName(state) {
      return state.player.name;
    },

    getNotification(state) {
      return state.notification;
    },

    isBailed(state) {
      return Boolean(state.player.bailedAt);
    },

    getPlayersActions(state) {
      return state.rooms[state.game.roomId]?.playersActions ?? [];
    },

    getPlayersStatus(state) {
      return state.game.playersStatus;
    },

    getTotalPlayers(state) {
      return state.game.playersStatus?.length;
    },

    getActivePlayers(state) {
      return state.game.playersStatus?.filter((player) =>
        [
          PLAYER_STATUS.JOINED_GAME.toLowerCase(),
          PLAYER_STATUS.LEFT_GAME.toLowerCase(),
        ].includes(player.player_status?.toLowerCase()),
      );
    },

    getWinnerName(state): string | null {
      return state.game.winner;
    },

    getWinnerPrize(state): string {
      return (
        "$" +
        (state.game.playersStatus?.length ?? 0) *
          (state.rooms?.find((room) => room.id === state.game.roomId)
            ?.entry_fee ?? 1) *
          0.95
      );
    },

    getWatchingPlayers() {
      return [];
    },

    getMessages(state): IMessage[] {
      return state.game.roomId != -1
        ? state.rooms[state.game.roomId].messages
        : [];
    },

    isAudioOn(state): boolean {
      return state.gameAudioMode;
    },

    isMusicOn(state): boolean {
      return state.gameMusicMode;
    },

    getChatMode(state): boolean {
      return state.chat.mode;
    },

    chatMessageText(state): string {
      return state.chat.text;
    },

    isMenuOpen(state): boolean {
      return state.menu.isOpen;
    },

    getCurrentGame(state): IGame | undefined {
      return state.rooms[state.game.roomId].games.find(
        (game) => game.id === state.game.id,
      );
    },

    getCurrentGameMint(state): string | undefined {
      return state.rooms[state.game.roomId].games.find(
        (game) => game.id === state.game.id,
      )?.mint;
    },

    getGameCoinCalc(state): number {
      return (
        Math.pow(
          10,
          state.rooms[state.game.roomId]?.games?.find(
            (game) => game.id === state.game.id,
          )?.mint == "So11111111111111111111111111111111111111112"
            ? 9
            : 6,
        ) ?? 9
      );
    },
  },

  mutations: {
    // SET_AUTH_CHAT_WS_NOTIFIER_WEBSOCKET(state, ws: WebSocket) {
    //   state.auth_chat_ws_notifier = ws;
    // },

    SET_UN_AUTH_GAME_WS_NOTIFIER_WEBSOCKET(state, ws: WebSocket) {
      state.game_ws_notifier = ws;
    },

    SET_PLAYING_AUDIO_FILE(state, playingAudioFile: string | null) {
      state.playingAudioFile = playingAudioFile;
    },

    SET_UN_AUTH_CHAT_WS_NOTIFIER_WEBSOCKET(state, ws: WebSocket) {
      state.unauth_chat_ws_notifier = ws;
    },

    SET_BALANCE(state, balance: number) {
      state.balance = balance;
    },

    SET_SHOW_DEPOSIT_MODAL(state, showDepositModal: DepositModalType) {
      state.showDepositModal = showDepositModal;
    },

    SET_SIGNED_MESSAGE(state, signedMessage: string) {
      state.signedMessage = signedMessage;
    },

    SET_LOGGED_IN(state, loggedIn: boolean) {
      state.loggedIn = loggedIn;
    },
    SET_USER_ID(state, userId: string) {
      state.user_id = userId;
    },

    SET_NONCE(state, nonce: string) {
      state.nonce = nonce;
    },

    SET_WALLET(state, wallet: Wallet) {
      state.wallet = wallet;
    },

    SET_WINNER(state, winner: string) {
      state.game.winner = winner;
    },

    DISCONNECT_WALLET(state) {
      state.wallet = null;
    },

    SET_GAME_AUDIO(state, mode: boolean) {
      state.gameAudioMode = mode;
    },

    SET_GAME_MUSIC(state, mode: boolean) {
      state.gameMusicMode = mode;
    },

    SET_WALLET_ADDRESS(state, wallet_address: string) {
      state.wallet_address = wallet_address;
    },

    SET_GAME_BUTTON_SHAKING_MODE(state, buttonShaking: boolean) {
      state.game.buttonShaking = buttonShaking;
    },

    SET_GAME(state, game: GetGameResponse) {
      state.game.id = game.id;
      let roomId =
        state.rooms.find((room) => room.currentGameId === game.id)?.id ?? -1;
      if (roomId === -1) {
        state.rooms.forEach((room) => {
          room.games.forEach((game) => {
            if (game.id === game.id) {
              roomId = room.id;
            }
          });
        });
      }

      state.game.roomId = roomId;
      state.game.playersStatus = game.players_statuses;
      if (![GameStatus.Settled].includes(game.game_status)) {
        state.game.status = game.game_status;
        state.game.counter_time = game.counter_end_time
          ? game.counter_end_time * 1000 - new Date().getTime()
          : null;
      }

      if (state.game.counter_time) {
        state.game.start_time = new Date();
        if (state.game.counter_time < 0) {
          state.game.counter_time = null;
        } else if (state.game.counter_time < 3) {
          state.game.aboutToStart = true;
        }
      }

      if (game.winner) {
        state.game.winner =
          game.players_statuses.find(
            (player) => player.wallet_address === game.winner,
          )?.name ?? null;
      }

      if (game.game_status === GameStatus.Finished && !state.game.winner) {
        state.game.winner = "THE HOUSE";
      }

      if (game.game_status === GameStatus.Settled) {
        state.game.start_time = null;
      }

      if (game.game_status === GameStatus.WaitingForPlayers) {
        state.game.winner = null;
      }

      if (game.game_status === GameStatus.OnGoing && game.start_time) {
        state.game.start_time = new Date(game.start_time * 1000);
        state.game.timer = true;
        state.game.aboutToStart = false;
      }

      // console.log("current game: ", state.game, game);
    },

    SET_GAME_MODE(state, mode: boolean) {
      state.game.live = mode;
    },

    TOGGLE_CHAT_MODE(state) {
      state.chat.mode = !state.chat.mode;
    },

    SET_GAME_TIMER_MODE(state, mode: boolean) {
      state.game.timer = mode;
    },

    SET_GAME_COUNTER_TIME(state, counter_time: number) {
      state.game.counter_time = counter_time;
    },

    SET_GAME_ABOUT_TO_START_MODE(state, mode: boolean) {
      state.game.aboutToStart = mode;
    },

    SET_PLAYER_PLAYING_MODE(state, mode: boolean) {
      state.player.playing = mode;
    },

    SET_NOTIFICATION(state, notification: INotification) {
      state.notification = notification;
    },

    SET_BAIL(state, date: Date) {
      state.player.bailedAt = date;
    },

    SET_PLAYER_NAME(state, name: string) {
      state.player.name = name;
    },

    ADD_PLAYER_ACTION(state, playerActionRecord: IPlayerActionRecord) {
      if (state.rooms[state.game.roomId].playersActions) {
        state.rooms[state.game.roomId].playersActions?.unshift(
          playerActionRecord,
        );
      } else {
        state.rooms[state.game.roomId].playersActions = [playerActionRecord];
      }
    },

    ADD_PLAYER_STATUS(state, playerStatusRecord: GamePlayersStatus) {
      if (state.game.playersStatus) {
        state.game.playersStatus?.unshift(playerStatusRecord);
      } else {
        state.game.playersStatus = [playerStatusRecord];
      }
    },

    UPDATE_PLAYER_STATUS(state, playerStatusRecord: GamePlayersStatus) {
      if (!state.game.playersStatus) {
        return;
      }

      const playerStatusRecordIndex = state.game.playersStatus.findIndex(
        (record) => record.wallet_address === playerStatusRecord.wallet_address,
      );
      if (playerStatusRecordIndex === -1) {
        return;
      }

      state.game.playersStatus[playerStatusRecordIndex] = playerStatusRecord;
    },

    REMOVE_PLAYER_STATUS(state, playerStatusRecordId: string) {
      if (!state.game.playersStatus) {
        return;
      }

      const playerStatusRecordIndex = state.game.playersStatus.findIndex(
        (record) => record.wallet_address === playerStatusRecordId,
      );
      if (playerStatusRecordIndex === -1) {
        return;
      }

      state.game.playersStatus = state.game.playersStatus.splice(
        playerStatusRecordIndex,
        1,
      );
    },

    ADD_CHAT_MESSAGE(state, chatMessage: IMessage) {
      state.rooms[chatMessage.roomId].messages.push(chatMessage);
    },

    UPDATE_CHAT_MESSAGE_TEXT(state, chatMessageText: string) {
      state.chat.text = chatMessageText;
    },

    TOGGLE_MENU_STATE(state) {
      state.menu.isOpen = !state.menu.isOpen;
    },

    SET_ROOMS(state, games: IGame[]) {
      games = games.sort((a, b) => a.entry_fee - b.entry_fee);
      games.forEach((game) => {
        let roomIndex = state.rooms.findIndex(
          (room) => room.entry_fee === game.entry_fee,
        );

        if (roomIndex === -1) {
          state.rooms.push({
            id: state.roomId,
            entry_fee: game.entry_fee,
            currentGameId: game.id,
            games: [game],
            messages: [],
            playersActions: [],
            watchingPlayers: 0,
          });

          state.roomId = state.roomId + 1;
          roomIndex = state.rooms.findIndex(
            (room) => room.entry_fee === game.entry_fee,
          );
        }

        const gameIndex = state.rooms[roomIndex].games.findIndex(
          (stateGame) => stateGame.id === game.id,
        );
        if (gameIndex === -1) {
          state.rooms[roomIndex].games.push(game);
        }

        if (game.game_status === GameStatus.WaitingForPlayers) {
          state.rooms[roomIndex].currentGameId = game.id;
          state.rooms[roomIndex].entry_fee = game.entry_fee;
        }
      });

      // console.log("state.rooms", state.rooms);
    },

    SET_CONNECTION(state) {
      state.connection = new Connection("https://api.devnet.solana.com");
      // state.connection = new Connection(
      //   "https://staging-rpc.dev2.eclipsenetwork.xyz",
      // );
    },

    CLOSE_CONNECTION(state) {
      state.connection = null;
    },

    SET_WINNER_MODE(state, mode: boolean) {
      state.game.winnerMode = mode;
    },

    SET_GAME_START_TIME(state, time: Date | null) {
      state.game.start_time = time;
    },

    SET_GAME_STATUS(state, status: GameStatus) {
      state.game.status = status;
    },

    SET_WEB_SOCKET(state: any, ws: { name: string; socket: WebSocket }) {
      state[ws.name] = ws.socket;
    },

    REMOVE_WEB_SOCKET(state: any, name: string) {
      state[name] = null;
    },

    // SET_PLAYER_WATCHING(state, roomId: number) {
    // state.rooms[roomId].
    // }
  },
  actions: {
    async connectWs(context) {
      try {
        // logger.info("connectWs starting");
        // const auth_sockets = ["auth_chat_ws_notifier"];
        const un_auth_sockets = ["unauth_chat_ws_notifier", "game_ws_notifier"];
        // for (const name of auth_sockets) {
        //   if (context.getters.isLoggedIn) {
        //     await context.dispatch("registerWebSocket", name);
        //   }
        // }
        for (const name of un_auth_sockets) {
          await context.dispatch("registerWebSocket", name);
        }
      } catch (error) {
        logger.error("connectWs", error);
      } finally {
        // logger.info("connectWs finally");
      }
    },

    async registerWebSocket(context: any, name: string) {
      try {
        if (context.state[name]) {
          const socket = context.state[name];
          if (
            socket.readyState === WebSocket.OPEN ||
            socket.readyState == WebSocket.CONNECTING
          ) {
            return;
          }
          socket.onclose = () => {
            logger.warn(`Websocket closed : ${name}`);
          };
          socket.close();
          context.commit("REMOVE_WEB_SOCKET", name);
          await sleep(300);
        }
        const socket = new WebSocket(`${baseWsOrigin}/${name}`);

        const pingInterval = setInterval(() => {
          if (socket.readyState === WebSocket.OPEN) {
            socket.send(JSON.stringify({ type: "ping" }));
          }
        }, 25_000);

        socket.addEventListener("error", (event) => {
          logger.error(`Websocket error : ${name}`, event);
        });

        socket.addEventListener("close", (event) => {
          logger.warn(`Websocket closed : ${name}`, event);
          clearInterval(pingInterval);
          if (socket) {
            socket.onclose = () => {
              logger.warn(`Websocket closed : ${name}`);
            };
            setTimeout(async () => {
              await context.dispatch("registerWebSocket", name);
            }, 1_000);
          }
        });

        socket.addEventListener("open", () => {
          socket.send(JSON.stringify({ type: "ping" }));
        });

        // Listen for messages
        socket.addEventListener("message", (event) => {
          context.dispatch("handleIncomingMessage", {
            name,
            message: event.data,
          });
        });

        context.commit("SET_WEB_SOCKET", { name, socket });
      } catch (error) {
        logger.error(`registerWebSocket: ${name}`, error);
      } finally {
        // logger.info(`registerWebSocket finally ${name}`);
      }
    },

    async getChainBalance(context) {
      context.commit("SET_CONNECTION");
      const wallet = context.state.wallet;
      const connection = context.state.connection;
      if (!wallet || !connection) {
        console.error("deposit: wallet is not connected");
        return;
      }
      const balance = await getWalletBalance(
        connection,
        wallet.publicKey(),
        NATIVE_MINT,
      );
      context.commit("SET_BALANCE", balance);
      // console.log("getChainBalance", balance);
      context.commit("CLOSE_CONNECTION");
    },

    playAudio(context, audio: string | null) {
      if (!context.getters.isAudioOn) {
        return;
      }

      context.commit("SET_PLAYING_AUDIO_FILE", audio);
    },

    async register(context) {
      try {
        // logger.info("register starting");
        const wallet = context.state.wallet;
        if (!wallet || !wallet.isConnected()) {
          logger.warn("register: wallet is not connected");
          return;
        }
        const registerResponse = await ApiRequests.register({
          wallet_address: wallet.publicKey().toBase58(),
        });
        // logger.info("registerResponse", registerResponse);
        if (registerResponse.isErr) {
          console.error("register", registerResponse.error);
          return;
        }
        context.commit("SET_USER_ID", registerResponse.unwrap().id);
        context.commit("SET_NONCE", registerResponse.unwrap().nonce);
        await sleep(300);
        // logger.info("Registered successfully");
      } catch (error) {
        logger.error("register", error);
      } finally {
        // logger.info("register finally");
      }
    },

    async login(context) {
      try {
        const wallet = context.state.wallet;
        const nonce = context.state.nonce;
        if (!wallet) {
          console.error("login: wallet is not connected");
          return;
        }
        if (!nonce) {
          console.error("login: nonce is not set");
          return;
        }
        const uint8Array = new TextEncoder().encode(nonce);
        const signed_message = await wallet.signMessage(uint8Array);
        const decodedString = bs58.encode(signed_message.signature);
        const response = await ApiRequests.login({
          wallet_address: wallet.publicKey().toBase58(),
          signed_message: decodedString,
          nonce,
        });

        if (response.isOk) {
          const session = {
            uuid: response.unwrap().uuid,
            signed_message: decodedString,
          };
          await context.dispatch("setUserAsLogged", session);
          await context.dispatch("storeSession", session);
          context.commit("SET_WALLET_ADDRESS", wallet.publicKey().toBase58());
          context.commit("SET_PLAYER_NAME", response.value.name);
          await context.dispatch("checkGameStatus");
          await context.dispatch("getChainBalance");
        } else {
          logger.error("login", response.error);
        }
      } catch (error) {
        logger.error("login", error);
      } finally {
        // logger.info("login finally");
      }
    },

    storeSession(_context, session) {
      sessionStorage.setItem("token", JSON.stringify(session));
    },

    setUserAsLogged(context, session) {
      context.commit("SET_USER_ID", session.uuid);
      context.commit("SET_SIGNED_MESSAGE", session.signed_message);
      context.commit("SET_LOGGED_IN", true);
    },

    async logout(context) {
      context.commit("SET_LOGGED_IN", false);
      context.commit("SET_USER_ID", null);
      context.commit("SET_SIGNED_MESSAGE", null);
      await ApiRequests.logout({
        wallet_address: context.getters.getWalletAddress,
      });
      context.state.wallet?.disconnect();
      context.commit("SET_WALLET", null);
      context.commit("SET_WALLET_ADDRESS", null);
    },

    async loginIfHasSession() {
      const token = sessionStorage.getItem("token");
      if (!token) {
        return;
      }

      // await context.dispatch("connectPhantom", (window as any)?.solana);
      // await context.dispatch("register");
      // context.dispatch('setUserAsLogged', JSON.parse(token));
      // await context.dispatch('connectWs');
    },

    async getGames(context) {
      const response = await ApiRequests.get_games();
      if (response.isOk) {
        context.commit("SET_ROOMS", response.unwrap().games);
        // console.log("getGames", JSON.stringify(response.unwrap(), null, 2));
      } else {
        console.error("getGames", response.error);
      }
    },

    async getGame(context, game_id: string) {
      const response = context.getters.isLoggedIn
        ? await ApiRequests.get_game({ game_id })
        : await ApiRequests.unauth_get_game({ game_id });
      if (response.isOk) {
        context.commit("SET_GAME", response.unwrap());
        // console.log("getGame", JSON.stringify(response.unwrap(), null, 2));
      } else {
        console.error("getGame", response.error);
      }
    },

    async deposit(context, amount: number) {
      context.commit("SET_CONNECTION");
      const wallet = context.state.wallet;
      const connection = context.state.connection;

      if (!wallet || !connection) {
        context.commit("SET_NOTIFICATION", {
          type: "error",
          content: "Connect your wallet to deposit money",
        });
        console.error("deposit: wallet is not connected");
        return;
      }

      try {
        const instructions = await playerProfileDepositInstruction({
          amount,
          signer: wallet.publicKey(),
          mint: NATIVE_MINT,
          connection,
        });

        const txn = await processTransactionWithWallet(
          instructions,
          connection,
          wallet,
        );
        if (txn && txn.Signature) {
          context.commit("SET_NOTIFICATION", {
            type: "info",
            content: "Your deposit completed successfully",
          });
          await context.dispatch("getChainBalance");
        }
      } catch (error) {
        context.commit("SET_NOTIFICATION", {
          type: "error",
          content: "Sorry, failed to completed your deposit",
        });
      }
      context.commit("CLOSE_CONNECTION");
    },

    async withdraw(context, amount: number) {
      console.log("here");

      context.commit("SET_CONNECTION");
      const wallet = context.state.wallet;
      const connection = context.state.connection;
      if (!wallet || !connection) {
        context.commit("SET_NOTIFICATION", {
          type: "error",
          content: "Connect your wallet to withdraw money",
        });
        return;
      }

      try {
        const instruction = playerProfileWithdrawInstruction({
          amount,
          signer: wallet.publicKey(),
          mint: NATIVE_MINT,
        });

        const txn = await processTransactionWithWallet(
          [instruction],
          connection,
          wallet,
        );

        if (txn && txn.Signature) {
          context.commit("SET_NOTIFICATION", {
            type: "info",
            content: "Your withdraw completed successfully",
          });
          await context.dispatch("getChainBalance");
        }
      } catch (error) {
        context.commit("SET_NOTIFICATION", {
          type: "error",
          content: "Sorry, failed to completed your withdraw",
        });
      }
      context.commit("CLOSE_CONNECTION");
    },

    async connectPhantom(context, wallet: any) {
      // logger.info("connectPhantom", wallet);
      try {
        context.commit("SET_WALLET", new Wallet(wallet));
        await context.state.wallet?.connect();
        if (context.state.wallet?.isConnected()) {
          // logger.info("connectPhantom", "Connected successfully");
        } else {
          logger.error("connectPhantom", "Failed to connect");
        }
      } catch (error) {
        logger.error("connectPhantom", error);
      } finally {
        // logger.info("connectPhantom finally", wallet);
        // context.dispatch('storeWalletConnection', wallet)
      }
    },

    // storeWalletConnection(context, wallet) {
    //   localStorage.setItem('brainwar', JSON.stringify(wallet));
    // },

    // checkIfAlreadyLogged(context) {
    //   const token = localStorage.getItem('brainwar');
    //   return token ? JSON.parse(token) : null;
    // },

    async leaveGameReal(context, game_id: string) {
      const wallet = context.state.wallet;
      const user_id = context.state.user_id;
      if (!wallet) {
        context.commit("SET_NOTIFICATION", {
          type: "error",
          content: "Connect your wallet to play the game",
        });
        console.error("leaveGame: wallet is not connected");
        return;
      }
      if (!user_id) {
        console.error("leaveGame: user_id is not set");
        return;
      }
      const response = await ApiRequests.leave_game({
        game_id,
      });
      if (response.isOk) {
        // console.log("leaveGame", response.unwrap());
      } else {
        console.error("leaveGame", response.error);
      }
    },

    async loadSettingsFromCache(context) {
      const isAudioOn = await context.dispatch("getCookie", "isAudioOn");
      context.commit("SET_GAME_AUDIO", isAudioOn == 1);
      const isMusicOn = await context.dispatch("getCookie", "isMusicOn");
      context.commit("SET_GAME_MUSIC", isMusicOn == 1);
    },

    setCookie(_context, { name, value, days }) {
      const date = new Date();
      date.setTime(date.getTime() + days * 24 * 60 * 60 * 1000);
      const expires = "expires=" + date.toUTCString();
      document.cookie = `${name}=${value}; expires=${expires}; path=/; Secure=true;`;
    },

    getCookie(_context, name) {
      return new Promise((resolve) => {
        const value = `; ${document.cookie}`;
        const parts = value.split(`; ${name}=`);
        if (parts.length === 2) {
          resolve(parts?.pop()?.split(";").shift());
        }
        resolve(null);
      });
    },

    async toggleGameAudio(context) {
      await context.dispatch("setCookie", {
        name: "isAudioOn",
        value: !context.getters.isAudioOn ? "1" : "0",
        days: 30,
      });
      context.commit("SET_GAME_AUDIO", !context.getters.isAudioOn);
    },

    async toggleGameMusic(context) {
      await context.dispatch("setCookie", {
        name: "isMusicOn",
        value: !context.getters.isMusicOn ? "1" : "0",
        days: 30,
      });
      await context.dispatch(
        "playAudio",
        !context.getters.isMusicOn ? "enable_sound" : "disable_sound",
      );
      context.commit("SET_GAME_MUSIC", !context.getters.isMusicOn);
    },

    async updatePlayerName(context, playerName: string) {
      const wallet = context.state.wallet;
      const user_id = context.state.user_id;
      if (!wallet || !context.state.wallet_address) {
        console.error("updatePlayerName: wallet is not connected");
        return;
      }
      if (!user_id) {
        console.error("updatePlayerName: user_id is not set");
        return;
      }

      if (playerName.length > 20) {
        await context.dispatch("playAudio", "notification");
        context.commit("SET_NOTIFICATION", {
          type: "error",
          content: "Username exceeds 20 characters limit",
        });
        return;
      }

      const response = await ApiRequests.update_user_name({
        name: playerName,
        wallet_address: context.state.wallet_address,
      });
      if (response.isOk) {
        context.commit("SET_PLAYER_NAME", playerName);
        await context.dispatch("playAudio", "notification");
        context.commit("SET_NOTIFICATION", {
          type: "info",
          content: `User name updated to "${playerName}"`,
        });
      } else {
        console.error("updatePlayerName", response.error);
        await context.dispatch("playAudio", "notification");
        context.commit("SET_NOTIFICATION", {
          type: "error",
          content: "User name already exists",
        });
      }
    },

    async joinGameReal(context, game_id: string) {
      const wallet = context.state.wallet;
      const user_id = context.state.user_id;
      if (!wallet) {
        console.error("joinRoom: wallet is not connected");
        return;
      }
      if (!user_id) {
        console.error("joinRoom: user_id is not set");
        return;
      }
      const response = await ApiRequests.join_game({
        game_id,
      });
      if (response.isOk) {
        if (
          response.value.message &&
          REQUIRES_DEPOSIT_MESSAGES.includes(response.value.message)
        ) {
          context.commit("SET_SHOW_DEPOSIT_MODAL", DepositModal.MISSING_MONEY);
          context.commit("SET_PLAYER_PLAYING_MODE", false);
          return;
        }
      } else {
        context.commit("SET_PLAYER_PLAYING_MODE", false);
        await context.dispatch("playAudio", "notification");
        context.commit("SET_NOTIFICATION", {
          type: "error",
          content: "Failed to join the game",
        });
        console.error("joinRoom", response.error);
      }
    },

    async setDepositModalMode(context, mode: DepositModalType) {
      if (!context.state.wallet && mode !== DepositModal.CLOSED) {
        await context.dispatch("playAudio", "notification");
        const action = DepositModal.WITHDRAW === mode ? "withdraw" : "deposit";
        context.commit("SET_NOTIFICATION", {
          type: "error",
          content: `Connect your wallet to ${action} money`,
        });
        console.error("joinRoom: wallet is not connected");
        return;
      }

      context.commit("SET_SHOW_DEPOSIT_MODAL", mode);
    },

    async setDepositPrice(context, price) {
      context.commit("SET_SHOW_DEPOSIT_MODAL", DepositModal.CLOSED);
      await context.dispatch(
        "deposit",
        price * context.getters.getGameCoinCalc,
      );
      if (context.getters.getCurrentRoom) {
        await context.dispatch(
          "joinGameReal",
          context.getters.getCurrentRoom.currentGameId,
        );
      }
    },

    async setWithdrawPrice(context, price) {
      context.commit("SET_SHOW_DEPOSIT_MODAL", DepositModal.CLOSED);
      await context.dispatch(
        "withdraw",
        price * context.getters.getGameCoinCalc,
      );
    },

    async joinRoom(context, roomId: number) {
      const room = context.state.rooms[roomId];
      if (!room) {
        return;
      }
      await context.dispatch("getGame", room.currentGameId);
      context.commit("SET_PLAYER_PLAYING_MODE", false);
      context.commit("SET_BAIL", null);
      await context.dispatch("checkGameStatus");
      await context.dispatch("connectWs");
    },

    checkGameStatus(context) {
      const playerStatus = context.state.game.playersStatus?.find(
        (player) => player.wallet_address === context.state.wallet_address,
      );
      if (!playerStatus) {
        return;
      }

      context.commit("SET_PLAYER_PLAYING_MODE", true);

      if (playerStatus.player_status === PLAYER_STATUS.LEFT_GAME) {
        context.commit("SET_BAIL", new Date());
      }
    },

    async startGame(context) {
      context.commit("SET_GAME_MODE", true);
      context.commit("SET_GAME_TIMER_MODE", true);
      context.commit("SET_GAME_ABOUT_TO_START_MODE", false);
      await context.dispatch("stopGame");
    },

    gameAboutToStart(context) {
      context.commit("SET_GAME_ABOUT_TO_START_MODE", true);
    },

    toggleChat(context) {
      context.commit("TOGGLE_CHAT_MODE");
    },

    async joinGame(context) {
      if (!context.state.game.id) {
        return;
      }

      if (!context.state.wallet) {
        await context.dispatch("playAudio", "notification");

        context.commit("SET_NOTIFICATION", {
          type: "info",
          content: "Connect your wallet to start playing",
        });

        context.commit("SET_GAME_BUTTON_SHAKING_MODE", true);
        setTimeout(() => {
          context.commit("SET_GAME_BUTTON_SHAKING_MODE", false);
        }, 800);
        return;
      }

      context.commit("SET_PLAYER_PLAYING_MODE", true);
      await context.dispatch(
        "joinGameReal",
        context.getters.getCurrentRoom.currentGameId,
      );
    },

    async bailGame(context) {
      if (context.state.game.roomId == -1) {
        return;
      }

      await context.dispatch("leaveGameReal", context.state.game.id);
      context.commit("SET_BAIL", new Date());
      context.commit("SET_PLAYER_PLAYING_MODE", false);
    },

    addPlayerActionRecord(context, playerActionRecord: IPlayerActionRecord) {
      context.commit("ADD_PLAYER_ACTION", playerActionRecord);
    },

    addPlayerStatusRecord(context, playerStatusRecord: GamePlayersStatus) {
      context.commit("ADD_PLAYER_STATUS", playerStatusRecord);
    },

    removePlayerStatusRecord(context, playerStatusRecordId: string) {
      context.commit("REMOVE_PLAYER_STATUS", playerStatusRecordId);
    },

    updateChatMessageText(context, chatMessageText: string) {
      context.commit("UPDATE_CHAT_MESSAGE_TEXT", chatMessageText);
    },

    async sendChatMessage(context, message: string) {
      if (!context.state.game.id) {
        // logger.info("sendChatMessage", "No game id");
        await context.dispatch("playAudio", "notification");
        context.commit("SET_NOTIFICATION", {
          type: "info",
          content: "Need to enter a game to use chat",
        });
        return;
      }

      if (!context.getters.isLoggedIn) {
        // logger.info("sendChatMessage", "Not logged in");
        await context.dispatch("playAudio", "notification");
        context.commit("SET_NOTIFICATION", {
          type: "info",
          content: "Connect your wallet to use the chat",
        });
        return;
      }

      if (
        context.state?.unauth_chat_ws_notifier?.readyState !== WebSocket.OPEN
      ) {
        // logger.info("sendChatMessage", "Websocket not open");
        return;
      }

      context.state?.unauth_chat_ws_notifier?.send(
        JSON.stringify({
          message,
          roomId: context.state.game.roomId,
          name: context.state.player.name,
        }),
      );
    },

    async handleIncomingMessage(context, data) {
      const message = JSON.parse(data.message);
      if (message.type == "pong") {
        return;
      }
      if (message.type === "ping") {
        if (data.name === "unauth_chat_ws_notifier") {
          context.state.unauth_chat_ws_notifier?.send(
            JSON.stringify({ type: "pong" }),
          );
        }
        if (data.name === "game_ws_notifier") {
          context.state.game_ws_notifier?.send(
            JSON.stringify({ type: "pong" }),
          );
        }
        return;
      }
      // logger.info("handleIncomingMessage", data);
      if (data.name === "unauth_chat_ws_notifier") {
        await context.dispatch("handleChatMessages", message);
      }
      if (data.name === "game_ws_notifier") {
        await context.dispatch("handleGameMessage", message);
      }
    },

    handleChatMessages(context, data) {
      context.commit("ADD_CHAT_MESSAGE", {
        name: data.name ?? "Guest",
        message: data.message,
        roomId: data.roomId,
        created_at: new Date(),
      });
    },

    async handleGameMessage(context, data) {
      console.log("handleGameMessage", data);
      if (data.msg_name == MsgName.NEW_GAME_CREATED) {
        await context.dispatch("getGames");
        context.dispatch(
          "getGame",
          context.getters.getCurrentRoom.currentGameId,
        );
        context.commit("SET_GAME_START_TIME", 0);
        return;
      }

      if (data.msg_name == MsgName.GAME_SETTLED) {
        console.log("here");

        setTimeout(async () => {
          console.log("here2");
          await context.dispatch(
            "getGame",
            context.getters.getCurrentRoom.currentGameId,
          );
        }, 5000);
      }

      if (context.state.game.id !== data.game_id) {
        return;
      }

      if (data.msg_name == MsgName.GAME_STARTED) {
        context.commit("SET_GAME_START_TIME", new Date());
        context.commit("SET_GAME_TIMER_MODE", true);
        context.commit("SET_GAME_STATUS", GameStatus.OnGoing);
        context.commit("SET_GAME_ABOUT_TO_START_MODE", false);
      } else if (data.msg_name == MsgName.GAME_ENDED) {
        context.commit("SET_WINNER_MODE", true);
        context.commit("SET_GAME_STATUS", GameStatus.Finished);
        context.commit("SET_GAME_START_TIME", null);
        context.commit("SET_GAME_TIMER_MODE", false);
        context.commit("SET_PLAYER_PLAYING_MODE", false);
        context.commit("SET_BAIL", null);
        // setTimeout(() => {
        //   context.commit("SET_WINNER_MODE", false);
        //   context.commit("SET_GAME_STATUS", GameStatus.WaitingForPlayers);
        //   context.dispatch(
        //     "getGame",
        //     context.getters.getCurrentRoom.currentGameId,
        //   );
        // }, 7000);
      } else if (data.msg_name == MsgName.GAME_COUNTER) {
        context.commit(
          "SET_GAME_COUNTER_TIME",
          data.start_time_from_now_in_milliseconds * 1000,
        );
        context.commit("SET_GAME_START_TIME", new Date());
      }

      if (PlayerActionMsgValues.includes(data.msg_name)) {
        context.commit("ADD_PLAYER_ACTION", {
          name: data.name ? data.name : "SYSTEM",
          action: PlayerActionMsgEnum[data.msg_name],
          created_at: new Date(),
        });
        await context.dispatch("getChainBalance");
      }

      if (!data.avoidFetchingGame) {
        await context.dispatch("getGame", data.game_id);
      }
    },

    toggleMenuState(context) {
      context.commit("TOGGLE_MENU_STATE");
    },
  },
  modules: {},
});
