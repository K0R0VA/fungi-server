use crate::schema::{client, project};
use crate::actor::db::PgActor;
//use crate::actor::ws::WsActor;

use diesel::{Queryable, Insertable, Associations};
use actix::{ Message, Addr };
use uuid::Uuid;
use actix::MailboxError;


use actix::prelude::{ Request, Handler };
use serde::{Deserialize, Serialize};
use actix_web::web::{Json};


pub struct AppState {
    pub pg : Addr<PgActor>,
//// TODO:
//  pub ws : Addr<WsActor>,
//  pub md : Addr<MdActor>
////
}


impl AppState {
    pub fn pg_send<'de, I : 'static, S : 'static>(&self, input : Json<I>) -> Request<PgActor, I>
    where I : Deserialize<'de>  + Send,
              PgActor: Handler<I>,
              <I as Message>::Result: Send,
              S : Serialize,
              I : Message<Result = Result<S, MailboxError>>  {
        self.pg.send(input.into_inner())
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable, PartialEq)]
#[table_name="client"]
pub struct Client {
    pub id : Uuid,
    pub name : String,
    pub email : String,
    pub password : String,
}

impl Client {
    pub fn new(name : String, email : String, password : String ) -> Self {
        Client {
            id : Uuid::new_v4(),
            name : name.clone(),
            email : email.clone(),
            password : password.clone()
        }
    }
}

impl Message for Client {
    type Result = Result<Vec<Project>, MailboxError>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Associations, PartialEq)]
#[table_name="project"]
pub struct Project {
    pub id : Uuid,
    pub name : String,
    pub path : String,
    pub client_id : Uuid
}

impl Project {
    pub fn new(name : String, client_id : Uuid) -> Self {
        Project {
            id : Uuid::new_v4(),
            name : name.clone(),
            path : format!("/{:?}/{}", client_id, name.clone()),
            client_id
        }
    }
    pub fn is_client_project(&self, client_id : Uuid) -> bool {
        self.client_id == client_id
    }
}


