use actix_web::{dev::Server, get, web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/greet/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/hello_world")]
async fn hello_world() -> impl Responder {
    "Hello World!"
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(greet)
            .service(hello_world)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
