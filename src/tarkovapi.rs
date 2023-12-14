use reqwest::{Error};
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::Deserialize;

const URL: &str = "https://api.tarkov.dev/graphql";

pub struct TarkovApi;

pub fn new_tarkov_api() -> TarkovApi {
    TarkovApi{}
}

impl TarkovApi {
    pub async fn get_ammo(&self) -> Result<Vec<Ammo>, Error> {
        let client = reqwest::Client::new();

        let body = r#"{
     "query": "{ ammo { item { name } damage penetrationPower penetrationChance tracer stackMaxSize caliber } }"
    }
    "#;
        println!("body: {}", body);
        let response = client.post(URL).header(CONTENT_TYPE, "application/json").header(ACCEPT, "application/json").body(body).send().await?;

        let parsed: TarkovApiResponse = response.json().await?;

        return Ok(parsed.data.ammo)
    }
}

#[derive(Deserialize, Debug)]
struct TarkovApiResponse {
    data: Data,
}

#[derive(Deserialize, Debug)]
struct Data {
    ammo: Vec<Ammo>,
}

#[derive(Deserialize, Debug)]
pub struct Ammo {
    item: Item,
    damage: u32,
    penetrationPower: u32,
    penetrationChance: f32,
    caliber: String,
    tracer: bool,
    stackMaxSize: u32,
}

#[derive(Deserialize, Debug)]
struct Item {
    name: String
}