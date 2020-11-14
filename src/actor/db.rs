use crate::model::schema::{client, project};
use crate::model::{ client::Client, project::Project };

use validator::Validate;
use actix::{Actor, SyncContext, Handler, Message, };
use actix::MailboxError;
use diesel::{prelude::*, r2d2::{ConnectionManager, Pool}, PgConnection};
use serde::Deserialize;
use uuid::Uuid;
use crate::middleware::crypto_service::CryptoService;
use crate::model::client::{NewUser, SignIn};
use crate::model::project::NewProject;
use crate::model::schema::client::columns::password;
use failure::Fail;

pub struct PgActor<'a> {
    pool :Pool<ConnectionManager<PgConnection>>,
    crypto : CryptoService<'a>
}

impl<'a> PgActor<'a> {
    pub fn new(pool : Pool<ConnectionManager<PgConnection>>, crypto : CryptoService<'static>) -> Self {
        PgActor {
            pool,
            crypto
        }

    }
}


impl Actor for PgActor<'static> {
    type Context = SyncContext<Self>;
}


impl Handler<NewUser> for PgActor<'static> {
    type Result = Result<Client, MailboxError>;

    fn handle(&mut self, msg: NewUser, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        let hash_pass = self.crypto.hash_password(msg.password);
        let client : Client = Client::new(&msg.username, &msg.email, &hash_pass);
        let result : Client = diesel::insert_into(client::table)
            .values::<Client>(client)
            .get_result(conn)
            .expect("SignUp Error");
        Ok(result)
    }
}

impl Handler<SignIn> for PgActor<'static> {
    type Result = Result<Client, MailboxError>;

    fn handle(&mut self, msg: SignIn, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        let client = client::table
            .filter(client::email.eq(msg.email))
            .first::<Client>(conn)
            .expect("SignIn Error");
        if self.crypto.verify_password(msg.password, &*client.password) {
            Ok(client)
        }
        else {
            Err(MailboxError::Closed)
        }
    }
}

// impl Handler<> for PgActor<'static> {
//
//     type Result = Result<Vec<Project>, MailboxError>;
//
//     fn handle(&mut self, client_id: Uuid, _ : &mut Self::Context) -> Self::Result {
//         let conn : &PgConnection = &self.pool.get().unwrap();
//         let projects = project::table
//             .filter(project::client_id.eq(client_id))
//             .load::<Project>(conn)
//             .expect("Error loading projects");
//         Ok(projects)
//     }
// }

impl Handler<NewProject> for PgActor<'static> {
    type Result = Result<Project, MailboxError>;

    fn handle(&mut self, msg: NewProject, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.pool.get().unwrap();
        let project = Project::new(&msg.name_project, msg.id_client);
        let result = diesel::insert_into(project::table)
            .values(project)
            .get_result(conn)
            .expect("New Project Error");
        Ok(result)
    }
}


