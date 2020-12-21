use super::AppContext;
use super::super::super::model::postgre::client::{SignIn, Creator};
use super::super::super::model::postgre::project::{Project, ClientIdForProjects};
use super::super::super::model::mongo::get_project::{GetProject, MongoProject};


use juniper::{FieldError, graphql_value };
use validator::Validate;
use crate::model::postgre::plugin::{Plugin, GetPlugins, GetPluginInfo, GetComments, Comment};

pub struct Query;

type FieldResult<T> = Result<T, FieldError>;

#[juniper::graphql_object(Context = AppContext)]
impl Query {

    pub async fn api_version() -> &str {
        "1.0"
    }

    pub async fn login_client(client: SignIn, context: &AppContext) -> FieldResult<Creator> {
        match Validate::validate(&client) {
            Ok(_) => {
                let password = client.password.clone();
                match context.database.pg_send(client).await  {
                    Ok(client) => verify(context.descriptor.verify_password(password, &client.password.clone()).await, client),
                    Err(_) => Err(FieldError::new(ERROR_MESSAGE, graphql_value!("")))
                }
            }
            Err(e) => {
                Err(FieldError::new(format!("{:?}", e), graphql_value!("")))
            }
        }
    }
    pub async fn get_client_projects(client_id: ClientIdForProjects, context: &AppContext) -> FieldResult<Vec<Project>> {
         context.database.pg_send(client_id).await
    }
    pub async fn get_project(id: GetProject, context: &AppContext) -> FieldResult<MongoProject> {
        context.database.mongo_send(id).await
    }
    pub async fn get_plugins(context: &AppContext) -> FieldResult<Vec<Plugin>> {
        context.database.pg_send(GetPlugins).await
    }
    pub async fn get_plugin(id: GetPluginInfo, context: &AppContext) -> FieldResult<Plugin> {
        context.database.pg_send(id).await
    }
    pub async fn get_messages(req: GetComments, context: &AppContext) -> FieldResult<Vec<Comment>> {
        context.database.pg_send(req).await
    }
}

const ERROR_MESSAGE: &str = "password or email not verified";

fn verify(condition: bool, client: Creator) -> Result<Creator, FieldError> {
    if condition {
        Ok(client)
    }
    else {
        Err(FieldError::new(ERROR_MESSAGE, graphql_value!("")))
    }
}

