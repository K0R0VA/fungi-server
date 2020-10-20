#![feature(arbitrary_self_types)]
#![feature(type_alias_impl_trait)]
#![feature(async_closure)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod schema;
mod model;
mod actor;
mod route;

use crate::actor::db::PgActor;
use crate::model::{ AppState, Client, Project};
use crate::actor::db::{SignUp, SignIn};
use crate::route::handler::get_responce;

use actix_web::{App, Responder, HttpServer, post, get};
use actix_web::web::{Data, Json};
use actix_web::web;
use actix::{Addr, SyncArbiter};
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;


use std::env;

async fn registration (state : Data<AppState>, sign_up : Json<SignUp>) -> impl Responder {
    get_responce(state, sign_up).await.unwrap()
}

async fn login (state : Data<AppState>, sign_up : Json<SignIn>) -> impl Responder {
    get_responce(state, sign_up).await.unwrap()
}

async fn projects ( state : Data<AppState>, client : Json<Client>) -> impl Responder {
    get_responce(state, client).await.unwrap()
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let config_url = env::var("CONFIGURATION_URL").expect("CONFIGURATION_URL is not set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build the Pool");
    let pg_addr : Addr<PgActor> = SyncArbiter::start(5, move || PgActor(pool.clone()));
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

