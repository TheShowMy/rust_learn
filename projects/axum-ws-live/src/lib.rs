use std::sync::Arc;

use axum::{
    Extension,
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use dashmap::{DashMap, DashSet};
use futures::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use tracing::warn;

const CAPACITY: usize = 64;

use crate::msg::Msg;

#[derive(Debug)]
struct State {
    //一个用户下的所有房间
    user_rooms: DashMap<String, DashSet<String>>,
    //一个房间下的所有用户
    room_users: DashMap<String, DashSet<String>>,
    //广播通道 使用subscribe来获取rx 用于接收消息
    tx: broadcast::Sender<Arc<Msg>>,
}

#[derive(Debug, Clone)]
pub struct ChatState(Arc<State>);

impl State {
    fn new() -> Self {
        let (tx, _rx) = broadcast::channel(CAPACITY);
        Self {
            user_rooms: DashMap::new(),
            room_users: DashMap::new(),
            tx,
        }
    }
}

impl ChatState {
    pub fn new() -> Self {
        Self(Arc::new(State::new()))
    }

    pub fn get_user_rooms(&self, username: &str) -> Vec<String> {
        self.0
            .user_rooms
            .get(username)
            .map(|rooms| rooms.clone().into_iter().collect())
            .unwrap_or_default()
    }

    pub fn get_room_users(&self, room: &str) -> Vec<String> {
        self.0
            .room_users
            .get(room)
            .map(|users| users.clone().into_iter().collect())
            .unwrap_or_default()
    }
}

mod msg;
pub async fn ws_handler(
    Extension(state): Extension<ChatState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: ChatState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.0.tx.subscribe();
    let state_clone = state.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(data)) = receiver.next().await {
            match data {
                Message::Text(msg) => {
                    handle_message(msg.as_str().try_into().unwrap(), state_clone.0.clone()).await;
                }
                _ => (),
            }
        }
    });
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let msg_str: String = msg.as_ref().try_into().unwrap();
            if sender.send(Message::Text(msg_str.into())).await.is_err() {
                break;
            }
        }
    });

    tokio::select! {
        _ = &mut recv_task => send_task.abort(),
        _ = &mut send_task => recv_task.abort(),
    }

    warn!("WebSocket 连接已关闭");

    
}

async fn handle_message(msg: Msg, state: Arc<State>) {
    let msg = match msg.message {
        crate::msg::MsgData::Join => {
            let username = msg.username.clone();
            let room = msg.room.clone();
            let room_users = state.room_users.entry(room.clone()).or_default();
            room_users.insert(username.clone());

            let user_rooms = state.user_rooms.entry(username.clone()).or_default();
            user_rooms.insert(room.clone());
            msg
        }
        crate::msg::MsgData::Leave => {
            let username = msg.username.clone();
            let room = msg.room.clone();
            if let Some(room_users) = state.room_users.get_mut(&room) {
                room_users.remove(&username);
                if room_users.is_empty() {
                    state.room_users.remove(&room);
                }
            }

            if let Some(user_rooms) = state.user_rooms.get_mut(&username) {
                user_rooms.remove(&room);
                if user_rooms.is_empty() {
                    state.user_rooms.remove(&username);
                }
            }
            msg
        }
        _ => msg,
    };
    let _ = state.tx.send(Arc::new(msg));
}
