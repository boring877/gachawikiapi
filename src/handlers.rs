use crate::types::ZoneNovaCharacter;
use worker::*;

pub async fn root(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("GachaWiki API - Welcome!")
}

pub async fn get_zone_nova_characters(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let json = include_str!("../data/zone-nova-characters.json");
    let characters: Vec<ZoneNovaCharacter> = serde_json::from_str(json).unwrap();
    Response::from_json(&characters)
}
