extern crate postgres;

use postgres::{Client, NoTls};

pub fn get_client() -> Client {
   Client::connect("host=localhost user=postgres password=000000 dbname=rust", NoTls).unwrap()
}

pub fn init_db(client: &mut Client) {
  client.simple_query(
            "
    CREATE TABLE if not exists blog (
        id      SERIAL PRIMARY KEY,
        title   VARCHAR(255) NOT NULL,
        body    TEXT
    )
",
        ).unwrap();
}
