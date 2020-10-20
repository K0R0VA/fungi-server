use crate::model::Project;
use crate::model::Client;

use actix::{Actor, Handler, Message, Context, Recipient};
use uuid::Uuid;

use std::collections::HashMap;

pub struct WsActor(HashMap<Uuid, Recipient<(Client, Vec<Project>)>>);

#[derive(Message)]
pub struct Connect {
    id : Uuid,
    addr : Recipient<(Client, Vec<Project>)>
}

#[derive(Message)]
pub struct Disconnect {
    id : Uuid
}

impl Actor for WsActor {
    type Context = Context<Self>;
}

impl Handler<Connect> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ : &mut Self::Context) -> Self::Result {
        self.0.insert(msg.id, msg.addr);
        Ok(())
    }
}

impl Handler<Disconnect> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ : &mut Self::Context) -> Self::Result {
        self.0.remove(&msg.id);
        Ok(())
    }
}

impl Handler<(Client, Vec<Project>)> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: (Client, Vec<Project>), ctx: &mut Self::Context) -> Self::Result {
        for ws in self.0.values() {
            ws.do_send(msg.clone());
        }
    }
}





