use uuid::Uuid;
use actix::{Message, MailboxError};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, Identifiable};
use validator::Validate;

use crate::model::schema::client;
use crate::model::project::Project;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[table_name="client"]
pub struct Client {
    pub id : Uuid,
    pub name : String,
    pub email : String,
    pub password : String,
}

impl Client {
    pub fn new(name : &str, email : &str, password : &str ) -> Self {
        Client {
            id : Uuid::new_v4(),
            name : name.to_owned(),
            email : email.to_owned(),
            password : password.to_owned()
        }
    }
}

impl Message for Client {
    type Result = Result<Vec<Project>, MailboxError>;
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct ClientId {

}

#[derive(Debug, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 6))]
    pub username : String,
    #[validate(email)]
    pub email : String,
    #[validate(must_match = "password")]
    pub password : String,
    #[validate(must_match(other = "password"))]
    pub password_check : String
}

impl Message for NewUser {
    type Result = Result<Client, MailboxError>;
}

#[derive(Deserialize, Validate)]
pub struct SignIn {
    #[validate(email)]
    pub email : String,
    pub password : String,
}

impl Message for SignIn {
    type Result = Result<Client, MailboxError>;
}
