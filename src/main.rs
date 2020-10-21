#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate validator_derive;

mod model;
mod actor;
mod routes;
mod utils;
mod middleware;
pub mod app_state;

use crate::actor::db::PgActor;
use app_state::AppState;
use crate::routes::{sign_in::login, sign_up::registration, projects::projects} ;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web;
use actix::SyncArbiter;
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;


use std::env;
use crate::middleware::auth_service::AuthService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let config_url = env::var("CONFIGURATION_URL").expect("CONFIGURATION_URL is not set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build the Pool");
    let pg_addr = SyncArbiter::start(5, move || PgActor(pool.clone()));
    HttpServer::new( move || App::new()
                                .data(AppState { pg : pg_addr.clone() })
                                .wrap(Logger::default())
                                //.wrap()
                                .route("signup/", web::post().to(registration) )
                                .route("signin/", web::get().to(login))
                                .route("projects/", web::post().to(projects))
    )
        .bind(config_url)?
        .run()
        .await
}

