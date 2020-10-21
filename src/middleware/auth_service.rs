use crate::app_state::AppState;
use actix_web::dev::{Service, ServiceRequest};
use actix::MailboxError;
use actix_web::web::Data;
use actix_web::HttpResponse;
use futures::Future;
use std::pin::Pin;
use futures::task::{Context, Poll};

pub struct AuthService {

}

impl Service for AuthService{
    type Request = ();
    type Response = ();
    type Error = ();
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        unimplemented!()
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        unimplemented!()
    }
}