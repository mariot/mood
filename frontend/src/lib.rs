#[macro_use]
extern crate seed;
extern crate dotenv;
use futures::Future;
use mood::{JsonApiResponse, Mood};
use seed::prelude::*;
use seed::{fetch, Request};
use std::env;

struct Model {
    moods: Vec<Mood>,
}

#[derive(Clone, Debug)]
enum Msg {
    FetchedMoods(fetch::ResponseDataResult<JsonApiResponse>),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedMoods(Ok(mut result)) => {
            model.moods.clear();
            model.moods.append(&mut result.data);
        }
        Msg::FetchedMoods(Err(reason)) => {
            log!(format!("Error fetching: {:?}", reason));
        }
    }
}

fn view(model: &Model) -> impl View<Msg> {
    let moods: Vec<Node<Msg>> = model
        .moods
        .iter()
        .map(|t| li![{ t.title.clone() }])
        .collect();

    h1![{ "Moods" }, ul![moods,],]
}

fn fetch_drills() -> impl Future<Item = Msg, Error = Msg> {
    let url = match env::var("BACKEND_URL") {
        Ok(url) => url,
        Err(_) => String::from("http://localhost:8000/moods/"),
    };
    Request::new(url).fetch_json_data(Msg::FetchedMoods)
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(fetch_drills());
    Model { moods: vec![] }
}

#[wasm_bindgen(start)]
pub fn render() {
    dotenv::dotenv().expect("Failed to read .env file");
    seed::App::build(init, update, view).finish().run();
}
