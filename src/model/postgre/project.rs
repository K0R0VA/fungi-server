use uuid::Uuid;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, QueryResult};
use chrono::{ Utc, NaiveDate};
use validator::Validate;
use juniper::{GraphQLInputObject, GraphQLObject};

use super::schema::project;
use actix::{Message};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, GraphQLObject)]
#[table_name="project"]
pub struct Project {
    pub id : Uuid,
    pub name : String,
    pub creator_id : Uuid,
    pub creation_data : NaiveDate,
    pub last_update : NaiveDate,
    pub definition: Option<String>,
}

impl Project {
    pub fn new(name : String, creator_id : Uuid) -> Self {
        Project {
            id : Uuid::new_v4(),
            name,
            creator_id,
            creation_data : Utc::now().date().naive_local(),
            last_update : Utc::now().date().naive_local(),
            definition: None
        }
    }
}

impl From<NewProject> for Project {
    fn from(project: NewProject) -> Self {
        Project {
            id : Uuid::new_v4(),
            name: project.name_project,
            creator_id: project.id_client,
            creation_data : Utc::now().date().naive_local(),
            last_update : Utc::now().date().naive_local(),
            definition: None
        }
    }
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


#[derive(Validate, GraphQLInputObject)]
pub struct NewProject {
    #[validate(length(min = 1, max = 50))]
    pub name_project : String,
    pub id_client : Uuid
}

impl Message for NewProject {
    type Result = QueryResult<Project>;
}

#[derive(GraphQLInputObject)]
pub struct ClientIdForProjects {
    pub id: Uuid
}

impl Message for ClientIdForProjects {
    type Result = QueryResult<Vec<Project>>;
}

pub(crate) struct SaveProject(pub Uuid);

impl Message for SaveProject {
    type Result = QueryResult<usize>;
}


