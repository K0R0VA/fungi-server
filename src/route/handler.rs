
use crate::model::{AppState};
use crate::PgActor;

use serde::{Deserialize, Serialize};
use actix::{MailboxError, Handler, Message};
use actix_web::{ HttpResponse};
use actix_web::web::{Data, Json};

use std::future::Future;

pub fn get_responce<'de, I : 'static, S : 'static>(state : Data<AppState>, input : Json<I> ) -> impl Future<Output = Result<HttpResponse, MailboxError>>
    where I : Deserialize<'de>  + Send,
          PgActor: Handler<I>,
          S : Serialize,
         <I as Message>::Result: Send,
          I : Message<Result = Result<S, MailboxError>>
{
    async move {
            let res = state.pg_send(input).await;
            res.and_then(
                |response | match response {
                    Ok(msgs) => Ok(HttpResponse::Ok().json(msgs)),
                    Err(_) => Ok(HttpResponse::InternalServerError().json("Internal Server Error")),
                }
            )
        }
}