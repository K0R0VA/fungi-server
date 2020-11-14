
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate validator;
extern crate lazy_static;

mod model;
mod actor;
mod routes;
mod utils;
mod middleware;
mod application;

use crate::actor::db::PgActor;
use crate::routes::{sign_in::login, sign_up::registration, projects::projects} ;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web;
use lazy_static::lazy_static;

use crate::application::app_state::AppState;
use crate::application::configuration::Config;

lazy_static! {
    static ref CONFIG : Config = Config::new();
 }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( move || App::new()
                                .data(AppState::new(CONFIG.database(), CONFIG.crypto_service()))
                                .wrap(Logger::default())
                                .service(projects)
                                .service(registration)
                                .service(login)
    )
        .bind(CONFIG.server())?
        .run()
        .await
}

