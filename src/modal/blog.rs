extern crate postgres;

use postgres::{Client};

pub struct Blog {
  pub title: String,
  pub body: String,
}


pub fn insert(client: &mut Client, title: &str, body: &str) -> u64 {
  let insert_sql = "insert into blog (title, body) values($1, $2)";
  let row_inserted = client.execute(insert_sql, &[&title, &body]).unwrap();
  return row_inserted;
}

pub fn query_all (client: &mut Client) {
  for row in client
        .query("SELECT id, body, body FROM blog", &[])
        .unwrap()
    {
        let id: i32 = row.get(0);
        let title: &str = row.get(1);
        let body: &str = row.get(2);

        println!("found person: {} {} {:?}", id, title, body);
    }
}
