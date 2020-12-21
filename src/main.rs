#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate validator;
extern crate juniper;
extern crate bson;

pub mod model;
pub mod actor;
pub mod middleware;
pub mod application;


use actix_web::{App, HttpServer, Result, web, guard, HttpResponse};
use actix_files as fs;
use application::descriptor::Descriptor;
use application::config::Configurator;
use application::database::DatabaseManager;
use middleware::graphql::graph_config;


async fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Configurator::default();
    let database = DatabaseManager::new(config.postgre(), config.mongo()).await;
    let descriptor = Descriptor::new(config.secret().to_string());
    HttpServer::new( move || App::new()
                                .data(database.clone())
                                .data(descriptor.clone())
                                .service(fs::Files::new("/static", "static/"))
                                .default_service(
                                    web::resource("")
                                        .route(web::get().to(index))
                                        .route(
                                            web::route()
                                                .guard(guard::Not(guard::Get()))
                                                .to(HttpResponse::MethodNotAllowed),
                                        )
                                )
                                .configure(graph_config)
    )
        .bind(config.server())?
        .run()
        .await
}

