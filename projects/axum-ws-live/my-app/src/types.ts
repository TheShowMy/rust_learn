export type MsgData =
  | "join"
  | "leave"
  | { message: string };

export interface ChatMessage {
  room: string;
  username: string;
  timestamp: number;
  message: MsgData;
}
