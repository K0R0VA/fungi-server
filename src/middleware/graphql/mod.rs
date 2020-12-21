mod mutation;
mod query;
mod schema;
mod subscription;

use super::super::application::database::DatabaseManager;
use super::super::middleware::graphql::schema::{create_schema, AppContext, Schema};

use actix_web::{HttpResponse, get, post };
use actix_web::web::{Json, Data, ServiceConfig, };
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use super::super::application::descriptor::Descriptor;


pub fn graph_config(config: &mut ServiceConfig) {
    let schema = create_schema();
    config
        .data(schema)
        .service(graphql)
        .service(graphiql);
}

#[get("/graphiql")]
async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql", Some("/graphql"));
    HttpResponse::Ok()
        .content_type( "text/html; charset=utf-8")
        .body(html)
}

#[post("/graphql")]
async fn graphql(data: Json<GraphQLRequest>, schema: Data<Schema>, database: Data<DatabaseManager>, descriptor: Data<Descriptor>) -> HttpResponse {
    let context = AppContext::new(database, descriptor);
    let result = data.execute(&schema, &context).await;
    HttpResponse::Ok().json(result)
}












