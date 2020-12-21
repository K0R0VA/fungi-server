use super::schema::{plugin, comment, liked, import};

use uuid::Uuid;
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, QueryResult};
use juniper::{ GraphQLObject, GraphQLInputObject };
use actix::Message;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, GraphQLObject)]
#[table_name="plugin"]
pub struct Plugin {
    pub id : Uuid,
    pub name : String,
    pub import_count: i32,
    pub creation_data : NaiveDate,
    pub last_update : NaiveDate,
    pub definition: Option<String>,
    pub public: bool,
    pub weight: f64,
    pub creator_id : Uuid,
}

impl Plugin {
    pub fn new(name : String, creator_id : Uuid, public: bool) -> Self {
        Plugin {
            id : Uuid::new_v4(),
            name,
            public,
            import_count: 0,
            creator_id,
            creation_data : Utc::now().date().naive_local(),
            last_update : Utc::now().date().naive_local(),
            definition: None,
            weight: 0.0
        }
    }
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct NewPlugin {
    pub name: String,
    pub creator: Uuid,
    pub public: bool
}

impl Message for NewPlugin {
    type Result = QueryResult<Plugin>;
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct DeletePlugin {
    pub id_plugin: Uuid
}

impl Message for DeletePlugin {
    type Result = QueryResult<usize>;
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct EditPlugin {
    pub id_plugin: Uuid,
    pub definition: String,
    pub public: bool,
    pub name: String
}

impl Message for EditPlugin {
    type Result = QueryResult<Plugin>;
}

pub struct GetPlugins;

impl Message for GetPlugins {
    type Result = QueryResult<Vec<Plugin>>;
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct GetPluginInfo {
    pub plugin_id: Uuid
}

impl Message for GetPluginInfo {
    type Result = QueryResult<Plugin>;
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, GraphQLObject)]
#[table_name="comment"]
pub struct Comment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plugin_id: Uuid,
    pub like_count: i32,
    pub content: String,
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct NewComment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plugin_id: Uuid,
    pub content: String
}

impl Message for NewComment {
    type Result = QueryResult<Comment>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, GraphQLObject)]
#[table_name="liked"]
pub struct Liked {
    pub id: Uuid,
    pub comment_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct Like {
    pub comment_id: Uuid,
    pub user_id: Uuid
}

impl Message for Like {
    type Result = QueryResult<usize>;
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct DisLike {
    pub comment_id: Uuid,
    pub user_id: Uuid
}

impl Message for DisLike {
    type Result = QueryResult<usize>;
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct GetComments {
    pub id_plugin: Uuid
}

impl Message for GetComments {
    type Result = QueryResult<Vec<Comment>>;
}

#[derive(Serialize, Deserialize, Queryable, Insertable, GraphQLObject)]
#[table_name="import"]
pub struct Import {
    pub id: Uuid,
    pub plugin_id: Uuid,
    pub project_id: Uuid
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct AddDependency {
    pub id_plugin: Uuid,
    pub id_project: Uuid
}

impl Message for AddDependency {
    type Result = QueryResult<usize>;
}

#[derive(Deserialize, Serialize, GraphQLInputObject)]
pub struct DeleteDependency {
    pub id_plugin: Uuid,
    pub id_project: Uuid
}

impl Message for DeleteDependency {
    type Result = QueryResult<usize>;
}