use crate::actor::postgres::PgActor;

use actix::{Addr, Message, MailboxError, Handler, SyncArbiter};
use serde::Serialize;
use diesel::r2d2::{ConnectionManager};
use diesel::{PgConnection, QueryResult};
use juniper::{FieldError, graphql_value};


#[derive(Clone)]
pub struct DatabaseManager {
    pub pg : Addr<PgActor>,
//// TODO:
//  pub ws : Addr<WsActor>,
//  pub md : Addr<MdActor>
////
}
impl DatabaseManager {
    pub fn new(db_addr : &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(db_addr);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to build the Pool");
        let pg = SyncArbiter::start(5,  move || {
            PgActor::new(pool.clone())
        });
        DatabaseManager {
            pg,
        }
    }

    pub async fn pg_send<'de, M: 'static, T: 'static>(&self, req : M) ->  Result<T, FieldError>
        where M: Message + Send,
              M::Result: Send,
              PgActor: Handler<M>,
              T: Serialize,
              M: Message<Result = QueryResult<T>>
    {
        let response: Result<QueryResult<T>, MailboxError>= self.pg.send(req).await;
        match response {
            Ok(r) => r.map_err(|e| FieldError::new(format!("{}", e), graphql_value!(""))),
            Err(e) => Err(FieldError::new(format!("{:?}", e), graphql_value!("")))
        }
    }
}
