use crate::types::Character;
use worker::*;

pub async fn root(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("GachaWiki API - Welcome!")
}

pub async fn get_characters(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    // Embed the data at compile time
    let json = include_str!("../data/characters.json");
    let characters: Vec<Character> = serde_json::from_str(json).unwrap();

    Response::from_json(&characters)
}
