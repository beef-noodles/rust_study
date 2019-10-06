// mod utils;
// mod modal;
// use utils::db::{get_client, init_db};
// use modal::blog::{Blog, insert as insert_blog, query_all as query_all_blog};

// fn run() {
//     let mut client = get_client();
//     init_db(&mut client);
//     let blog = Blog{
//         title: "test".to_string(),
//         body: "testst".to_string()
//     };
//     let row_inserted = insert_blog(&mut client, &blog.title, &blog.body);
//     println!("{}", row_inserted);
//     query_all_blog(&mut client);
// }



use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
    // run();
    HttpServer::new(|| {
        App::new().service(
            web::scope("api")
                .route("/", web::get().to(index))
                .route("/again", web::get().to(index2))
                .service(index3)
                // .route("/query", web::get().to(index_query)) 与下面的service等价，需要在index_query 上面加入宏
                .service(index_query)
                .route("json", web::post().to(index_json))
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
