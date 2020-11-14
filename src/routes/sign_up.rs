use actix_web::web::{Data, Json};
use actix_web::Responder;
use actix_web::post;

use crate::utils::responce::get_response;
use crate::application::app_state::AppState;
use crate::model::client::NewUser;

#[post("/auth/")]
pub async fn registration(state : Data<AppState>, sign_up : Json<NewUser>) -> impl Responder {
    get_response(state, sign_up).await.expect("login error")
}