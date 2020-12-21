pub mod client;
pub mod project;
pub mod schema;
pub mod plugin;

use juniper::{GraphQLObject};

#[derive(GraphQLObject)]
pub struct Status {
 pub inner: i32
}