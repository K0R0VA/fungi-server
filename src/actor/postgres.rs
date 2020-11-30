use crate::model::schema::{client, project};
use crate::model::{ client::Client, project::Project };
use crate::model::client::{ SignIn, NewUser, UserIdForProject};

use actix::{Actor, SyncContext, Handler };
use diesel::{prelude::*, r2d2::{ConnectionManager, Pool}, PgConnection};
use crate::model::project::NewProject;

pub struct PgActor {
    pool :Pool<ConnectionManager<PgConnection>>,
}

impl PgActor {
    pub fn new(pool : Pool<ConnectionManager<PgConnection>>) -> Self {
        PgActor {
            pool,
        }
    }
}



impl Actor for PgActor {
    type Context = SyncContext<Self>;
}

impl Handler<NewUser> for PgActor {
    type Result = QueryResult<Client>;

    fn handle(&mut self, msg: NewUser, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        let client : Client = Client::new(&msg.username, &msg.email, &msg.password);
        diesel::insert_into(client::table)
            .values::<Client>(client)
            .get_result(conn)
    }
}

impl Handler<SignIn> for PgActor {
    type Result = QueryResult<Client>;

    fn handle(&mut self, msg: SignIn, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        client::table
            .filter(client::email.eq(msg.email))
            .first::<Client>(conn)
    }
}

impl Handler<UserIdForProject> for PgActor {

    type Result = QueryResult<Vec<Project>>;

    fn handle(&mut self, msg: UserIdForProject, _ : &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        project::table
            .filter(project::client_id.eq(msg.0))
            .load::<Project>(conn)
    }
}

impl Handler<NewProject> for PgActor {
    type Result = QueryResult<Project>;

    fn handle(&mut self, msg: NewProject, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        let project = Project::new(msg.name_project, msg.id_client);
        diesel::insert_into(project::table)
            .values::<Project>(project)
            .get_result(conn)
    }
}


