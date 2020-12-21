use super::AppContext;
use super::super::super::model::postgre::client::{Creator, InputNewUser, NewUser};
use super::super::super::model::postgre::project::{NewProject, Project, SaveProject as IdSavedProject};
use super::super::super::model::mongo::input_project::{NewProject as MongoIdProject, SaveProject};

use juniper::{ FieldError, graphql_value};
use validator::{Validate};
use super::super::super::model::postgre::Status;
use crate::model::postgre::plugin::{NewPlugin, Plugin, NewComment, Comment};

pub struct Mutation;

type FieldResult<T> = Result<T, FieldError>;

#[juniper::graphql_object(Context = AppContext)]
impl Mutation {
    pub async fn create_client(input: InputNewUser, context: &AppContext) -> FieldResult<Creator> {
        match Validate::validate(&input) {
            Ok(_) => {
                let hash_pass = context.descriptor.hash_password(input.password).await;
                let new_user = NewUser::new((&input.username, &input.email), &hash_pass);
                context.database.pg_send(new_user).await
            }
            Err(e) => {
                Err(FieldError::new(format!("{:?}", e), graphql_value!("")))
            }
        }
    }
    pub async fn add_project(project: NewProject, context: &AppContext) -> FieldResult<Project> {
        match Validate::validate(&project) {
            Ok(_) => {
                let project = context.database.pg_send(project).await;
                map(project,context).await
            },
            Err(e) => {
                Err(FieldError::new(format!("{:?}", e), graphql_value!("")))
            }
        }
    }
    pub async fn save_project(project: SaveProject, context: &AppContext) -> FieldResult<Status> {
        let id = project.id;
        context.database.mongo_send(project).await.ok();
        context.database.pg_send(IdSavedProject {0 : id}).await.map(|code| Status { inner: code as i32} )
    }
    pub async fn add_plugin(plugin: NewPlugin, context: &AppContext) -> FieldResult<Plugin> {
        context.database.pg_send(plugin).await
    }
    pub async fn add_comment(comment: NewComment, context: &AppContext) -> FieldResult<Comment> {
        context.database.pg_send(comment).await
    }

}

async fn map(result: Result<Project, FieldError>, context: &AppContext) -> FieldResult<Project> {
    match result {
        Ok(project) => {
            match context.database.mongo_send(MongoIdProject { 0: project.id }).await {
                Ok(_) => Ok(project),
                Err(e) => Err(FieldError::new(format!("{:?}", e), graphql_value!("")))
            }
        },
        Err(e) => Err(e)
    }
}