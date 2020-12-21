use uuid::Uuid;
use actix::{Message};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, Identifiable, QueryResult};
use validator::Validate;
use juniper::{GraphQLObject, GraphQLInputObject};

use super::schema::creator;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable, GraphQLObject)]
#[table_name="creator"]
pub struct Creator {
    pub id : Uuid,
    pub name : String,
    pub email : String,
    pub bio: Option<String>,
    pub password : String,
}

impl Creator {
    pub fn new(name : &str, email : &str, password : &str) -> Self {
        Creator {
            id : Uuid::new_v4(),
            name : name.to_owned(),
            email : email.to_owned(),
            password : password.to_owned(),
            bio: None
        }
    }
}

impl PartialEq for Creator {
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
    pub fn new(input: (&str, &str), hashed_pass: &str) -> Self {
        NewUser {
            username: input.0.to_owned(),
            email: input.1.to_owned(),
            password: hashed_pass.to_owned()
        }
    }
}


impl Message for NewUser {
    type Result = QueryResult<Creator>;
}

#[derive(Deserialize, Validate, GraphQLInputObject)]
pub struct SignIn {
    #[validate(email)]
    pub email : String,
    pub password : String,
}

impl Message for SignIn {
    type Result = QueryResult<Creator>;
}

