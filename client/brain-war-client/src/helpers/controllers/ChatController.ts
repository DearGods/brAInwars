import BaseController from "./BaseController";

class ChatController {
  async sendMessage(roomId: string, data: object) {
    await BaseController.post(`chat/${roomId}/last-messages`, data);
  }

  async getMessages(roomId: string) {
    await BaseController.get(`chat/${roomId}/last-messages`);
  }
}

export default new ChatController();
