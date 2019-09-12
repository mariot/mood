#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use backend::db::{establish_connection, query_mood};
use mood::JsonApiResponse;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

#[get("/moods")]
fn moods_get() -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![] };

    let conn = establish_connection();
    for db_mood in query_mood(&conn) {
        let api_mood = mood::Mood {
            id: db_mood.id,
            title: db_mood.title,
        };
        response.data.push(api_mood);
    }

    Json(response)
}

fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .mount("/", routes![moods_get])
        .attach(cors)
        .launch();

    Ok(())
}
