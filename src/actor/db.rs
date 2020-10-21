use crate::model::schema::{client, project};
use crate::model::{ client::Client, project::Project };

use actix::{Actor, SyncContext, Handler, Message, };
use actix::MailboxError;
use diesel::{prelude::*, r2d2::{ConnectionManager, Pool}, PgConnection};
use serde::Deserialize;
use uuid::Uuid;


pub struct PgActor (pub Pool<ConnectionManager<PgConnection>>);

#[derive(Debug, Deserialize)]
pub struct SignUp {
    //#[validate(length(min = 6))]
    pub username : String,
    pub email : String,
    pub password : String
}


impl Actor for PgActor {
    type Context = SyncContext<Self>;
}


impl Message for SignUp {
    type Result = Result<Client, MailboxError>;
}

impl Handler<SignUp> for PgActor {
    type Result = Result<Client, MailboxError>;

    fn handle(&mut self, msg: SignUp, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.0.get().unwrap();
        let client : Client = Client::new(&msg.username, &msg.email, &msg.password);
        let result : Client = diesel::insert_into(client::table)
            .values::<Client>(client)
            .get_result(conn)
            .expect("SignUp Error");
        Ok(result)
    }
}
#[derive(Deserialize)]
pub struct SignIn {
    pub email : String,
    pub password : String
}

impl Message for SignIn {
    type Result = Result<Client, MailboxError>;
}

impl Handler<SignIn> for PgActor {
    type Result = Result<Client, MailboxError>;

    fn handle(&mut self, msg: SignIn, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.0.get().unwrap();
        let client = client::table
            .filter(client::password.eq(msg.password))
            .filter(client::email.eq(msg.email))
            .first::<Client>(conn)
            .expect("SignIn Error");
        Ok(client)
    }
}

impl Handler<Client> for PgActor {

    type Result = Result<Vec<Project>, MailboxError>;

    fn handle(&mut self, client: Client, _ : &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.0.get().unwrap();
        let projects = project::table
            .filter(project::client_id.eq(client.id))
            .load::<Project>(conn)
            .expect("Error loading projects");
        Ok(projects)
    }
}

pub struct NewProject {
    name_project : String,
    id_client : Uuid
}

impl Message for NewProject {
    type Result = Result<Project, ()>;
}

impl Handler<NewProject> for PgActor {
    type Result = Result<Project, ()>;

    fn handle(&mut self, msg: NewProject, _: &mut Self::Context) -> Self::Result {
        let conn : &PgConnection = &self.0.get().unwrap();
        let project = Project::new(&msg.name_project, msg.id_client);
        let result = diesel::insert_into(project::table)
            .values(project)
            .get_result(conn)
            .expect("New Project Error");
        Ok(result)
    }
}


