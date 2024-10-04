import BaseController from "./BaseController";

class GameController {
  async getPlayersStatus(roomId: string) {
    await BaseController.get(`game/${roomId}/players-status`);
  }

  async getPlayersActions(roomId: string) {
    await BaseController.get(`game/${roomId}/players-actions`);
  }

  async join(roomId: string) {
    await BaseController.post(`game/${roomId}/bail`);
  }

  async bail(roomId: string) {
    await BaseController.post(`game/${roomId}/bail`);
  }
}

export default new GameController();
