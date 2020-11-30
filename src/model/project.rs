use uuid::Uuid;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, Associations, QueryResult};
use chrono::{ Utc, NaiveDate};

use crate::model::schema::project;
use actix::{Message};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Associations)]
#[table_name="project"]
pub struct Project {
    pub id : Uuid,
    pub name : String,
    pub mongo_id : i32,
    pub client_id : Uuid,
    pub creation_data : NaiveDate,
    pub last_update : NaiveDate,
}

impl Project {
    pub fn new(name : String, client_id : Uuid) -> Self {
        Project {
            id : Uuid::new_v4(),
            name,
            mongo_id : 0,
            creation_data : Utc::now().date().naive_local(),
            last_update : Utc::now().date().naive_local(),
            client_id
        }
    }
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct NewProject {
    pub name_project : String,
    pub id_client : Uuid
}

impl Message for NewProject {
    type Result = QueryResult<Project>;
}
