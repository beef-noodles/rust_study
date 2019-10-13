#[macro_use]
extern crate dotenv_codegen;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result, middleware::Logger};
use serde::{Deserialize, Serialize};
use env_logger;



mod model;
mod controller;
use controller::auth::{auth};

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    userid: u32,
    friend: String,
}

fn index_path(info: web::Path<Info>) -> Result<String> {
    Ok(format!("parameter from path: Welcom {}, userid: {}!", info.friend, info.userid))
}

#[get("/query")]
fn index_query(web::Query(info): web::Query<Info>) -> Result<String> {
    Ok(format!("parameter from query: Welcome {}, userid: {}", info.friend, info.userid))
}

fn index_json(info: web::Json<Info>) -> Result<String> {
    Ok(format!("parameter from json: Welcome {}, userid: {}", info.friend, info.userid))
}

fn index_form(info: web::Form<Info>) -> Result<String> {
    Ok(format!("parameter from form: Welcome {}, userid: {}", info.friend, info.userid))
}
fn index_path_and_query ((path, web::Query(query)): (web::Path<(u32, String)>, web::Query<Info>)) -> Result<String> {
    println!("{}", "test");
    Ok(format!("parameter from path and qeury :Welcome {}, friend{}, userid: {}", query.friend, path.1, path.0))
}

fn response_json(info: web::Path<Info>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Info{
        userid: info.userid,
        friend: info.friend.to_string(),
    }))
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello, world! again")
}

use actix_web::get;

#[get("/hello")]
fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn main() {
    std::env::set_var("RUST_LOG", dotenv!("RUST_LOG"));
    env_logger::init();
    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
        .service(
            web::scope("api")
                .route("/", web::get().to(index))
                .route("/login", web::post().to(auth))
                .route("/again", web::get().to(index2))
                .service(index3)
                // .route("/query", web::get().to(index_query)) 与下面的service等价，需要在index_query 上面加入宏
                .service(index_query)
                .route("json", web::post().to(index_json))
                .route("responseJson/{userid}/{friend}", web::get().to(response_json))
                .route("form", web::post().to(index_form))
                .route("pathAndQuery/{id}/{name}", web::get().to(index_path_and_query))
                .route("users/{userid}/{friend}", web::get().to(index_path)),
        )
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap()
}
