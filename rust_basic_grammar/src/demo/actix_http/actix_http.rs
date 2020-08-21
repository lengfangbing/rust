use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyJson {
    name: String,
    age: i64,
}
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn hello(path: web::Path<String>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(MyJson {
        name: path.to_owned(),
        age: 22,
    }))
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