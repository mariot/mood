#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use mood::db::models::Mood;
use mood::db::{establish_connection, query_mood};
use rocket_contrib::json::Json;

#[derive(Serialize)]
struct JsonApiResponse {
    data: Vec<Mood>,
}

#[get("/moods")]
fn moods_get() -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![] };

    let conn = establish_connection();
    for mood in query_mood(&conn) {
        response.data.push(mood);
    }

    Json(response)
}

fn main() {
    rocket::ignite().mount("/", routes![moods_get]).launch();
}
