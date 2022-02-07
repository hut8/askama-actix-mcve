use actix_web::{get, App, HttpResponse, HttpServer};
use askama_actix::{Template, TemplateToResponse};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[get("/")]
async fn index() -> HttpResponse {
    IndexTemplate {}.to_response()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
