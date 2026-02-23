use crate::clock;
use crate::types::ZoneNovaCharacter;
use worker::*;

/// Check API key from Authorization header or X-API-Key header
/// Returns Ok(()) if authorized, Err if not
fn verify_api_key(req: &Request, env: &Env) -> Result<()> {
    // Get expected API key from environment
    let expected_key = match env.secret("API_KEY") {
        Ok(key) => key.to_string(),
        Err(_) => return Ok(()), // No API key configured, allow all requests
    };

    // Check X-API-Key header first
    if let Some(key) = req.headers().get("X-API-Key")? {
        if key == expected_key {
            return Ok(());
        }
    }

    // Check Authorization: Bearer header
    if let Some(auth) = req.headers().get("Authorization")? {
        if let Some(token) = auth.strip_prefix("Bearer ") {
            if token == expected_key {
                return Ok(());
            }
        }
    }

    Err(worker::Error::RustError("Unauthorized".to_string()))
}

/// Helper to return 401 if auth fails
fn auth_response(req: &Request, env: &Env) -> Option<Result<Response>> {
    if verify_api_key(req, env).is_err() {
        return Some(Response::error("Unauthorized: Invalid or missing API key", 401));
    }
    None
}

pub async fn root(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("GachaWiki API - Welcome!")
}

pub async fn get_zone_nova_characters(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if let Some(resp) = auth_response(&req, &ctx.env) {
        return resp;
    }
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
pub async fn get_clocks(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if let Some(resp) = auth_response(&req, &ctx.env) {
        return resp;
    }
    let clocks = clock::get_all_clocks();
    Response::from_json(&clocks)
}

/// GET /clock/:game_id - Returns clock for a specific game
pub async fn get_game_clock(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    if let Some(resp) = auth_response(&req, &ctx.env) {
        return resp;
    }
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
