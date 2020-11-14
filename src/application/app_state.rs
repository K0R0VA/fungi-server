use crate::actor::db::PgActor;
use actix::{Addr, Message, MailboxError, Handler, SyncArbiter};
use serde::{Deserialize, Serialize};
use actix_web::web::Json;
use actix::prelude::Request;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use crate::middleware::crypto_service::CryptoService;
use uuid::Uuid;

pub struct AppState {
    pub pg : Addr<PgActor<'static>>,
//// TODO:
//  pub ws : Addr<WsActor>,
//  pub md : Addr<MdActor>
////
}
impl AppState {
    pub fn new(db_addr : &str, crypto : CryptoService<'static>) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(db_addr);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to build the Pool");
        let pg = SyncArbiter::start(5, move || PgActor::new(pool.clone(), crypto.clone()));
        AppState {
            pg
        }
    }

    pub fn pg_send<'de, I : 'static, S : 'static>(&self, input : Json<I>) -> Request<PgActor<'static>, I>
        where I : Deserialize<'de>  + Send,
              PgActor<'static>: Handler<I>,
              <I as Message>::Result: Send,
              S : Serialize,
              I : Message<Result = Result<S, MailboxError>>  {
        self.pg.send(input.into_inner())
    }
}
