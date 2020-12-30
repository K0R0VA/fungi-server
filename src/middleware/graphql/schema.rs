use super::super::super::application::database::State;
use super::super::super::application::descriptor::Descriptor;
use super::query::Query;
use super::mutation::Mutation;

use actix_web::web::Data;
use juniper::{RootNode, Context};

pub struct AppContext {
    pub database: Data<State>,
    pub descriptor: Data<Descriptor>
}

impl AppContext {
    pub fn new(database: Data<State>, descriptor: Data<Descriptor>) -> Self {
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