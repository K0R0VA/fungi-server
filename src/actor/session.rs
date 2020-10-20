use crate::actor::ws::{Connect, Disconnect};
use crate::model::{AppState, Msg};

use actix::{Actor, Handler, Running, StreamHandler};
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};
use std::time::{Duration, Instant};


pub struct Session (uuid, Instant);

impl Actor for Session {
    type Context = WebSocketContext<Self, AppState>;
}

impl Session {

}