use super::super::actor::postgres::PgActor;

use actix::{Addr, Message, MailboxError, Handler, SyncArbiter, Actor, Context};
use diesel::r2d2::{ConnectionManager};
use diesel::{PgConnection, QueryResult};
use juniper::{FieldError, graphql_value};
use super::super::actor::mongo::MongoActor;
use std::fmt::Debug;


#[derive(Clone)]
pub struct DatabaseManager {
     postgre: Addr<PgActor>,
     mongo : Addr<MongoActor>,
//// TODO:
//  pub ws : Addr<WsActor>,
////
}
impl DatabaseManager {
    pub async fn new(postgre_address: &str, mongo_address: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(postgre_address);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to build the Pool");
        let postgre = SyncArbiter::start(5,  move || {
            PgActor::new(pool.clone())
        });
        let client = mongodb::Client::with_uri_str(mongo_address).await.unwrap();
        let mongo = MongoActor::create( |_: &mut Context<MongoActor>| {
            MongoActor {
                0: client,
            }
        });
        DatabaseManager {
            postgre,
            mongo
        }
    }

    pub async fn pg_send<'de, M: 'static, T: 'static>(&self, req : M) ->  Result<T, FieldError>
        where M: Message + Send,
              M::Result: Send,
              PgActor: Handler<M>,
              M: Message<Result = QueryResult<T>>
    {
        let result: Result<QueryResult<T>, MailboxError> = self.postgre.send(req).await;
        Self::map(result)
    }
    pub async fn mongo_send<'de, M: 'static, T: 'static, E: 'static>(&self, req : M) -> Result<T, FieldError>
        where M: Message<Result = Result<T, E>> + Send,
              T: Send,
              MongoActor: Handler<M>,
              E: Send + Debug,
    {
        let result = self.mongo.send(req).await;
        Self::map(result)
    }

    fn map<T, E: Debug>(res: Result<Result<T, E>, MailboxError>) -> Result<T, FieldError> {
        match res {
            Ok(r) => r.map_err(|e| FieldError::new(format!("{:?}", e), graphql_value!(""))),
            Err(e) => Err(FieldError::new(format!("{:?}", e), graphql_value!("")))
        }
    }
}
