
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate validator;
extern crate juniper;

mod model;
mod actor;
mod middleware;
mod application;

use crate::application::database::DatabaseManager;
use crate::middleware::graphql::graph_config;

use actix_web::{App, HttpServer, Result, web, guard, HttpResponse};
use actix_files as fs;
use std::env;
use dotenv::dotenv;
use application::descriptor::Descriptor;

async fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_addr = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let server_addr = env::var("CONFIGURATION_URL").expect("CONFIGURATION_URL is not set");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY is not set");
    let database = DatabaseManager::new(&*db_addr);
    let descriptor = Descriptor::new(secret_key);
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
                                                .to(|| HttpResponse::MethodNotAllowed()),
                                        )
                                )
                                .configure(graph_config)
    )
        .bind(server_addr)?
        .run()
        .await
}

