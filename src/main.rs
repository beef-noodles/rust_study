mod utils;
mod modal;
use utils::db::{get_client, init_db};
use modal::blog::{Blog, insert as insert_blog, query_all as query_all_blog};

pub fn main() {
    let mut client = get_client();
    init_db(&mut client);
    let blog = Blog{
        title: "test".to_string(),
        body: "testst".to_string()
    };
    let row_inserted = insert_blog(&mut client, &blog.title, &blog.body);
    println!("{}", row_inserted);
    query_all_blog(&mut client);
}
