extern crate postgres;

use postgres::{Connection, TlsMode};

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {
    println!("Hello, world!");
    let url = "postgresql://postgres:000000@localhost:5432/rust";
    let conn = Connection::connect(url, TlsMode::None).unwrap();
    // println!("{:?}", conn);
    let me = Person {
        id: 0,
        name: String::from("guzhongren"),
        data: None
    };

    let rows_updated = conn.execute("create table IF NOT EXISTS person (
        id              SERIAL PRIMARY KEY,
        name            VARCHAR NOT NULL,
        data            BYTEA
    )", &[]).unwrap();
    println!("{}", rows_updated);
    let rows_updated = conn.execute("insert into person(name, data) values($1, $2)", &[&me.name, &me.data]).unwrap();
    println!("{}", rows_updated );

    let all = conn.query("select id, name, data from person", &[]).unwrap();
    for row in &all {
        let person = Person{
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        };
        println!("Person: {}", person.name);
    }
    
}
