use crate::application::database::DatabaseManager;
use crate::application::descriptor::Descriptor;
use crate::middleware::graphql::query::Query;
use crate::middleware::graphql::mutation::Mutation;

use actix_web::web::Data;
use juniper::{RootNode, Context};

pub(crate) struct AppContext {
    pub(crate) database: Data<DatabaseManager>,
    pub(crate) descriptor: Data<Descriptor>
}

impl AppContext {
    pub(crate) fn new(database: Data<DatabaseManager>, descriptor: Data<Descriptor>) -> Self {
        AppContext {
            database,
            descriptor
        }
    }
}

impl Context for AppContext {}

pub(crate) type Schema = RootNode<'static, Query, Mutation, juniper::EmptySubscription<AppContext>>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(Query, Mutation, juniper::EmptySubscription::new() )
}