mod message;

use std::sync::Arc;

use axum::{extract::WebSocketUpgrade, async_trait, extract::{FromRequestParts, ws::{WebSocket, Message}}, TypedHeader, headers::{Authorization, authorization::Bearer}, http::{request::Parts, StatusCode}, response::IntoResponse, Extension};
use dashmap::{DashMap, DashSet};
use serde::{de::DeserializeOwned, Serialize, Deserialize};
use jsonwebtoken as jwt;

use message::{Msg, MsgData};
use tokio::sync::broadcast;
use futures::{SinkExt, StreamExt, Sink, Stream};
use tracing::warn;

// secret key
const SECRET_KEY: &[u8] = b"deadbeer";
const CAPACITY: usize = 64;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonClaim<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for CommonClaim<T>
where
    S: Send + Sync,
    T: DeserializeOwned + 'static
{
    type Rejection = HttpError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| HttpError::UnAuthorized)?;
        let key = jwt::DecodingKey::from_secret(SECRET_KEY);
        let token = jwt::decode::<T>(bearer.token(), &key, &jwt::Validation::default())
            .map_err(|_| HttpError::UnAuthorized)?;

        // Ok(token.claims)
        Ok(CommonClaim(token.claims))
    }
}

#[derive(Debug)]
pub enum HttpError {
    UnAuthorized,
    InternalServerError,
}

impl IntoResponse for HttpError {
  fn into_response(self) -> axum::response::Response {
      let (code, msg) = match self {
          HttpError::UnAuthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
          HttpError::InternalServerError => {
              (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
          }
      };
      (code, msg).into_response()
  }
}

// ======== ws handler ========
#[derive(Debug)]
pub struct State {
    user_rooms: DashMap<String, DashSet<String>>,
    room_users: DashMap<String, DashSet<String>>,
    tx: broadcast::Sender<Arc<Msg>>
}

impl Default for State {
    fn default() -> Self {
        let (tx, _rx) = broadcast::channel(CAPACITY);
        Self {
            user_rooms: Default::default(),
            room_users: Default::default(),
            tx
        }
    }
}
    
#[derive(Debug, Default, Clone)]
pub struct ChatState(Arc<State>);

impl ChatState {
    pub fn new() -> Self {
        Self(Default::default())
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

pub async fn ws_handler(ws: WebSocketUpgrade, Extension(state): Extension<ChatState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket<S>(socket: S, state: ChatState) 
  where S: Stream<Item = Result<Message, axum::Error>> + Sink<Message> + Send + 'static,
  {
    let mut rx = state.0.tx.subscribe();
    let (mut sender, mut receiver) = socket.split();
    let state1 = state.clone();

    let mut socket_task = tokio::spawn(async move {
        while let Some(Ok(data)) = receiver.next().await {
            match data {
                Message::Text(msg) => {
                    handle_message(msg.as_str().try_into().unwrap(), state1.0.clone()).await;
                }
                _ => {}
            }
        }
    });

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let data = msg.as_ref().try_into().unwrap();
            if sender.send(Message::Text(data)).await.is_err() {
                warn!("failed to send message");
                break;
            }
        }
    });

    tokio::select! {
        _v1 = &mut socket_task => send_task.abort(),
        _v2 = &mut send_task => socket_task.abort(),
    }

    warn!("connection closed");
    let username = "mz";
    for info in state.get_user_rooms(username) {
        if let Err(e) = state.0.tx.send(Arc::new(Msg::leave(&info, username))) {
            warn!("send message error: {}", e);
        }
    }
}

async fn handle_message(msg: Msg, state: Arc<State>) {
    let msg = match msg.data {
        MsgData::Join => {
            state.user_rooms.entry(msg.username.clone()).or_insert_with(DashSet::new).insert(msg.room.clone());
            state.room_users.entry(msg.room.clone()).or_insert_with(DashSet::new).insert(msg.username.clone());
            msg
        },
        MsgData::Leave => {
            if let Some(v) = state.user_rooms.get_mut(&msg.username) {
                v.remove(&msg.room);
                if v.is_empty() {
                    drop(v);
                    state.user_rooms.remove(&msg.username);
                }
            }
            if let Some(v) = state.room_users.get_mut(&msg.room) {
                v.remove(&msg.username);
                if v.is_empty() {
                    drop(v);
                    state.room_users.remove(&msg.room);
                }
            }
            msg
        },
        _ => msg
    };
    if let Err(e) = state.tx.send(Arc::new(msg)) {
        warn!("send message error: {}", e);
    }
}


// ============== test =============
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Result, Ok};
    use fake_socket::*;

    #[tokio::test]
    async fn handle_socket_should_work() -> Result<()> {
        let (_client1, _client2, state) = prepare_connections().await?;

        // verify state
        let mut users = state.get_room_users("lobby");
        users.sort();
        assert_eq!(users, &["alice", "tyr"]);

        let rooms = state.get_user_rooms("tyr");
        assert_eq!(rooms, &["lobby"]);

        Ok(())
    }

    #[tokio::test]
    async fn handle_message_and_leave_should_work() -> Result<()> {
        let (mut client1, mut client2, state) = prepare_connections().await?;
        
        let msg1 = &Msg::new("lobby", "tyr", MsgData::Message("hello world".into()));
        client1.send(Message::Text(msg1.try_into()?))?;

        verify(
            &mut client1,
            "lobby",
            "tyr",
            MsgData::Message("hello world".into()),
        )
        .await?;

        verify(
            &mut client2,
            "lobby",
            "tyr",
            MsgData::Message("hello world".into()),
        )
        .await?;

        let msg2 = &Msg::new("lobby", "tyr", MsgData::Leave);
        client1.send(Message::Text(msg2.try_into()?))?;

        assert!(client1.recv().await.is_some());
        assert!(client2.recv().await.is_some());

        // verify state
        let users = state.get_room_users("lobby");
        assert_eq!(users, &["alice"]);

        let rooms = state.get_user_rooms("tyr");
        assert!(rooms.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn handle_client_disconnect_should_work() -> Result<()> {
        let (client1, mut client2, _state) = prepare_connections().await?;
        drop(client1);

        verify(&mut client2, "lobby", "tyr", MsgData::Leave).await?;

        Ok(())
    }

    async fn prepare_connections() -> Result<(FakeClient<Message>, FakeClient<Message>, ChatState)> {
        let (mut client1, socket1) = create_fake_connection();
        let (mut client2, socket2) = create_fake_connection();

        let state = ChatState::new();
        // mimic server behavior
        let state1 = state.clone();
        tokio::spawn(async move {
            handle_socket(socket1, state1).await;
        });

        let state1 = state.clone();
        tokio::spawn(async move {
            handle_socket(socket2, state1).await;
        });

        let msg1 = &Msg::join("lobby", "tyr");
        client1.send(Message::Text(msg1.try_into()?))?;

        let msg2 = &Msg::join("lobby", "alice");
        client2.send(Message::Text(msg2.try_into()?))?;

        // should first get tyr join msg
        verify(&mut client1, "lobby", "tyr", MsgData::Join).await?;
        verify(&mut client2, "lobby", "tyr", MsgData::Join).await?;

        // then get alice join msg
        assert!(client1.recv().await.is_some());
        assert!(client2.recv().await.is_some());

        Ok((client1, client2, state))
    }

    async fn verify(client: &mut FakeClient<Message>, room: &str, username: &str, data: MsgData) -> Result<()> {
        if let Some(Message::Text(msg)) = client.recv().await {
            let msg = Msg::try_from(msg.as_str())?;
            assert_eq!(msg.room, room);
            assert_eq!(msg.username, username);
            assert_eq!(msg.data, data);
        };
        Ok(())
    }
  }