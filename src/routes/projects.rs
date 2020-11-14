use actix_web::web;
use actix_web::Responder;
use actix_web::get;
use uuid::Uuid;

use crate::model::client::Client;
use crate::utils::responce::get_response;
use crate::application::app_state::AppState;

#[get("/{user}/projects/")]
pub async fn projects (state : web::Data<AppState>, id : web::Path<Uuid> ) -> impl Responder {
    //get_response(state, id).await.unwrap()
    "".to_string()
}
