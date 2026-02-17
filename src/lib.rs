mod clock;
mod handlers;
mod types;
use handlers::*;
use worker::*;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let router = Router::new()
        .get_async("/", root)
        .get_async("/zone-nova/characters", get_zone_nova_characters)
        // Clock endpoints for Discord bot
        .get_async("/clock", get_clocks)
        .get_async("/clock/:game_id", get_game_clock);

    router.run(req, env).await
}
