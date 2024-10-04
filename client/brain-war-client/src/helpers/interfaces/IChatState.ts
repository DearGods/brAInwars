import IMessage from "./IMessage";

export default interface IChatState {
  text: string;
  mode: boolean;
  messages: null | Array<IMessage>;
}
