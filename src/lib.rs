mod handlers;
mod types;
use handlers::*;
use worker::*;

//using worker entry point for cloudflare workers

#[event(fetch)]

//CTX  we dont use it for now, it just extra worker for like send nofitcation

async fn fetch(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    // url router
    let router = Router::new()
        .get_async("/", root)
        .get_async("/characters", get_characters);

    router.run(req, env).await
}
