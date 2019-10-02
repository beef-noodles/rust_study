extern crate postgres;
mod db;

// use postgres::{Connection, TlsMode};

use db::*;

// struct Blog {
//     title: String,
//     body: String,
// }
fn main() {
    let conn = connect();
    
    // let blog = Blog{
    //     title: String::from("title"),
    //     body: String::from("body"),
    // };
    // let title = blog.title.to_string();
    // let body = blog.body.to_string();
    // insert_blog(&conn, &title, &body);
    // for row in query::<String>(&conn, "select * from blog") {
    //     print!("{:?}", row);
    // }
    // let sql = "select * from person";
    // query_all(&conn, &sql);
}
