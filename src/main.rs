use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{name}")]
async fn short(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("hello {}", name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(short)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}