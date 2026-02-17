use crate::clock;
use crate::types::ZoneNovaCharacter;
use worker::*;

pub async fn root(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("GachaWiki API - Welcome!")
}

pub async fn get_zone_nova_characters(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let json = include_str!("../data/zone-nova-characters.json");
    let characters: Vec<ZoneNovaCharacter> = match serde_json::from_str(json) {
        Ok(chars) => chars,
        Err(e) => {
            return Response::error(format!("JSON parse error: {}", e), 500);
        }
    };
    Response::from_json(&characters)
}

/// GET /clock - Returns all game clocks with countdown timers
pub async fn get_clocks(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let clocks = clock::get_all_clocks();
    Response::from_json(&clocks)
}

/// GET /clock/:game_id - Returns clock for a specific game
pub async fn get_game_clock(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let game_id = match ctx.param("game_id") {
        Some(id) => id.as_str(),
        None => return Response::error("Missing game_id parameter", 400),
    };

    match clock::get_game_clock(game_id) {
        Some(game_clock) => Response::from_json(&game_clock),
        None => {
            let available = clock::get_game_ids().join(", ");
            Response::error(
                format!("Game '{}' not found. Available games: {}", game_id, available),
                404,
            )
        }
    }
}
