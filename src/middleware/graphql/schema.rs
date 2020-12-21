use super::super::super::application::database::DatabaseManager;
use super::super::super::application::descriptor::Descriptor;
use super::query::Query;
use super::mutation::Mutation;

use actix_web::web::Data;
use juniper::{RootNode, Context};

pub struct AppContext {
    pub database: Data<DatabaseManager>,
    pub descriptor: Data<Descriptor>
}

impl AppContext {
    pub fn new(database: Data<DatabaseManager>, descriptor: Data<Descriptor>) -> Self {
        AppContext {
            database,
            descriptor
        }
    }
}

impl Context for AppContext {}

pub type Schema = RootNode<'static, Query, Mutation, juniper::EmptySubscription<AppContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, juniper::EmptySubscription::new() )
}