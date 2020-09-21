use actix_web::{get, web, App, HttpServer, Responder, post, HttpRequest};


#[post("/")]
async fn authorization() -> String {
    "a".to_owned()
}


#[get("/{user_name}/projects")]
async fn projects() -> impl Responder {
    format!("your projects!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(projects)
    )
        .bind("[::]::8088")?
        .run()
        .await
}
