use actix_web::web::{Data, Json};
use actix_web::Responder;
use actix_web::get;

use crate::utils::responce::get_response;
use crate::application::app_state::AppState;
use crate::model::client::SignIn;

#[get("/auth/")]
pub async fn login (state : Data<AppState>, sign_up : Json<SignIn>) -> impl Responder {
    get_response(state, sign_up).await.unwrap()
}