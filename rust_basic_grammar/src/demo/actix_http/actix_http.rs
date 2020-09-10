use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct MyJson {
    name: String,
    age: i64,
}
#[get("/{name}/{age}")]
async fn dynamic_test(info: web::Path<(String, u8)>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(MyJson {
        name: info.0.to_owned(),
        age: info.1 as i64,
    }))
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(dynamic_test)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}