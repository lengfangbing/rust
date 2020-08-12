use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

async fn hello(path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", &path))
}

#[actix_rt::main]
pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/{name}", web::get().to(hello))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}