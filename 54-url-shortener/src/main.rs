#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate rusqlite;

use std::sync::Mutex;
use rocket_contrib::Template;
use rocket::{Rocket, State};
use rusqlite::{Connection, Error};

type DbConn = Mutex<Connection>;

#[derive(Serialize)]
struct FormTemplate {
    urls: Vec<UrlRecord>
}

#[derive(Serialize)]
struct ShortUrlTemplate {
    short_url: String
}

#[derive(Debug,Serialize)]
struct UrlRecord {
    id: i32,
    url: String
}

#[get("/")]
fn index(db_conn: State<DbConn>) -> Template {
    let mut stmt = db_conn.lock().expect("db connection lock").prepare("SELECT id, url FROM urls").unwrap();
    let rec_iter = stmt.query_map(&[], |row| {
        UrlRecord {
            id: row.get(0),
            url: row.get(1)
        }
    }).unwrap();

    let mut urls: Vec<UrlRecord> = vec![];

    for rec in rec_iter {
        urls.push(rec.unwrap());
    }

    let context = FormTemplate {
        urls: urls
    };

    Template::render("form", &context)
}

#[post("/", format = "application/x-www-form-urlencoded")]
fn shorten() -> Template {
    // TODO: should pull some out into module

    /* TODO
        - save url (generate id)
        - convert id to short url
    */


    let context = ShortUrlTemplate {
        short_url: "http://locahost:8000/asdfasdf".to_string()
    };

    Template::render("short", &context)
}

fn main() {
    // Open a new in-memory SQLite database.
    let conn = Connection::open_in_memory().expect("in memory db");

    // Initialize the `entries` table in the in-memory database.
    init_database(&conn);

    rocket::ignite()
        .manage(Mutex::new(conn))
        .mount("/", routes![index, shorten])
        .attach(Template::fairing())
        .launch();
}

fn init_database(conn: &Connection) {
    conn.execute("CREATE TABLE urls (id INTEGER PRIMARY KEY, url TEXT NOT NULL UNIQUE)", &[])
        .expect("create entries table");
}

/*
 - form that accepts a long url
 - generate short url and store it with long in db
 - redirect visitors to long url when short visited
 - track the number of visits
 - provide stats page for urls
*/