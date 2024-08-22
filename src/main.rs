use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "./static").index_file("login.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}