use actix_web::{error, middleware, get, post, web, web::Bytes, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use std::str;


#[derive(Serialize, Deserialize)]
struct MyJson {
    name: String,
    age: i64,
}

#[derive(Deserialize)]
struct MyQuery {
    name: String,
    password: String,
}

#[derive(Deserialize)]
struct FormEncoded {
    name: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct JsonLoginHandle {
    name: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct MyJsonHandle {
    name: String,
    password: String,
    id: String,
    _type: String,
}

#[get("/query")]
async fn query_route(query: web::Query<MyQuery>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(MyJsonHandle{
        name: query.0.name.to_string(),
        password: query.0.password.to_string(),
        id: String::from("no id"),
        _type: String::from("query"),
    }))
}

#[get("/{name}/{age}")]
async fn dynamic_route(info: web::Path<(String, u8)>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(MyJson {
        name: info.0.to_owned(),
        age: info.1 as i64,
    }))
}

#[post("/{id}/login/json")]
async fn post_route_json(item: web::Json<JsonLoginHandle>, info: web::Path<String>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(MyJsonHandle{
        name: item.0.name,
        password: item.0.password,
        id: info.to_owned(),
        _type: String::from("json"),
    }))
}

#[post("/{id}/login/form")]
async fn post_route_form(form: web::Form<FormEncoded>, info: web::Path<String>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(MyJsonHandle{
        name: form.0.name,
        password: form.0.password,
        id: info.to_owned(),
        _type: String::from("form"),
    }))
}

#[post("/{id}/login/form_data")]
async fn post_route_form_data(mut payload: Multipart, info: web::Path<String>) -> Result<HttpResponse> {
    let mut body: Vec<(String, String)> = vec![];
    let mut file: Vec<(String, Bytes)> = vec![];
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition().unwrap();
        if let Some(name) = content_disposition.get_name() {
            if let Some(_) = content_disposition.get_filename() {
                while let Some(chunk) = field.next().await {
                    file.push((String::from(name), chunk.unwrap()));
                }
                continue;
            }
            while let Some(chunks) = field.next().await {
                body.push((String::from(name), String::from(str::from_utf8(&*chunks.unwrap()).unwrap())));
            }
        }
    }
    println!("{:?}", body);
    println!("{:?}", file);
    Ok(HttpResponse::Ok().json(MyJsonHandle{
        name: "form.0.name".to_owned(),
        password: "form.0.password".to_owned(),
        id: info.to_owned(),
        _type: String::from("form_data"),
    }))
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
pub async fn start() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(query_route)
            .service(index)
            .service(dynamic_route)
            .service(post_route_json)
            .service(post_route_form)
            .service(post_route_form_data)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
