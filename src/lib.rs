use actix_web::{get, web, App, HttpResponse, Responder, HttpServer, dev::Server};

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

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .service(health_check)
                .service(greet)
                .service(hello_world)
        })
        .bind(("127.0.0.1", 8080))?
        .run();

    Ok(server)
}
