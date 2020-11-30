use uuid::Uuid;
use actix::{Message, MailboxError};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, Identifiable, QueryResult};
use validator::Validate;
use juniper::{GraphQLObject, GraphQLInputObject};

use crate::model::schema::client;
use crate::model::project::Project;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable, GraphQLObject)]
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


#[derive(Debug, Deserialize, Validate, GraphQLInputObject)]
pub struct InputNewUser {
    #[validate(length(min = 6))]
    pub username : String,
    #[validate(email)]
    pub email : String,
    #[validate(must_match = "password")]
    pub password : String,
    #[validate(must_match(other = "password"))]
    pub password_check : String
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub username : String,
    pub email : String,
    pub password : String,
}

impl NewUser {
    pub(crate) fn new(input: (&str, &str), hashed_pass: &str) -> Self {
        NewUser {
            username: input.0.to_owned(),
            email: input.1.to_owned(),
            password: hashed_pass.to_owned()
        }
    }
}


impl Message for NewUser {
    type Result = QueryResult<Client>;
}

#[derive(Deserialize, Validate, GraphQLInputObject)]
pub struct SignIn {
    #[validate(email)]
    pub email : String,
    pub password : String,
}

impl Message for SignIn {
    type Result = QueryResult<Client>;
}

pub struct UserIdForProject(pub Uuid);


impl Message for UserIdForProject {
    type Result = QueryResult<Vec<Project>>;
}
