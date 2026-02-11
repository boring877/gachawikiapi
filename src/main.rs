// we are going to build a API server for our gachawiki, so we can pull that data from it !
// we will start small then go bigger !
// the first step we need to get the raw data from the dump files decrpyting them first
// these are general plan ! we dont need to worry for now about the dump or raw data !

//lets build our main fn using tokio !

mod types;
mod handlers;


use axum::{Router, routing::get};



async fn handler() -> &'static str {
    "GachaWiki API - Welcome!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()

        // our routes url
        .route("/", get(handler))
        .route("/api/characters", get(handlers::get_characters));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
