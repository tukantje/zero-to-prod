use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/greet/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello_world", web::get().to(|| async { "Hello World!" }))
            .service(health_check)
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
