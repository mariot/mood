extern crate dotenv;

use diesel::{prelude::*, sqlite::SqliteConnection};
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv::dotenv().expect("Failed to read .env file");
    let db = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => String::from("./testdb.sqlite3"),
    };
    SqliteConnection::establish(db.as_str())
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_mood<'a>(connection: &SqliteConnection, title: &'a str) {
    let mood = models::NewMood { title };

    diesel::insert_into(schema::mood::table)
        .values(&mood)
        .execute(connection)
        .expect("Error inserting new mood");
}

pub fn query_mood(connection: &SqliteConnection) -> Vec<models::Mood> {
    schema::mood::table
        .load::<models::Mood>(&*connection)
        .expect("Error loading moods")
}
