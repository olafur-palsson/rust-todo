use axum::{routing::get, Router};
use axum::response::{IntoResponse, Html};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use render::{html};

mod entities;
mod components;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/list", get(list))
        .route("/comp_test", get(comp_test))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn comp_test() -> Html<String> {
    let html = html! {
        <h1>{"Heading"}</h1>
    };
    Html(html)
}

async fn list() -> &'static str  {
    "<h1>Test works</h1>"
}
