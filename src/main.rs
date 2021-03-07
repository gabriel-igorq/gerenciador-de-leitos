use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

async fn ping() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async  fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(ping))
            .route("/", web::get().to(ping))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}