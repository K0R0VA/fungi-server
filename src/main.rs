#[macro_use]
extern crate diesel;
extern crate dotenv;

mod schema;
mod model;
mod actor;
mod route;
mod utils;

use crate::actor::db::PgActor;
use crate::model::app_state::AppState;
use crate::actor::db::{SignUp, SignIn};
use crate::route::{ sign_in::login, sign_up::registration, projects::projects} ;

use actix_web::{App, Responder, HttpServer};
use actix_web::web;
use actix::SyncArbiter;
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;


use std::env;


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
                                .route("signup/", web::post().to(registration) )
                                .route("signin/", web::get().to(login))
                                .route("projects/", web::post().to(projects))
                                //.route("", mut route: Route)
    )
        .bind(config_url)?
        .run()
        .await
}

