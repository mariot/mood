#[macro_use]
extern crate serde;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mood {
    pub id: i32,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonApiResponse {
    pub data: Vec<Mood>,
}
