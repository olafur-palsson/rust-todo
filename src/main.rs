#![allow(unused_braces)]
use axum::{routing::get, Router};
use yew::{function_component, html, ServerRenderer};
use axum::response::Html;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use crate::app::todo::types::{TodoItem, TodoList};
use crate::ui::todo::todo_list::TodoComp;

mod ui;
mod app;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/list", get(list))
        .route("/heading_test", get(heading_test))
        .route("/page_test", get(page_test))
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

async fn page_test() -> Html<String> {
    Html("yolo".to_string())
    // Html(html! {
    //     <h1>{heading}</h1>
    // })
}

async fn heading_test() -> Html<String> {
    Html("yolo".to_string())
    // Html(html! {
    //     <h1>{heading}</h1>
    // })
}


async fn comp_test() -> Html<String> {
    let renderer = ServerRenderer::<App>::new();
    let result = renderer.render().await; 
    Html(result)
}



#[function_component]
pub fn App() -> yew::Html {
    let todo2 = TodoItem {
        id: String::from("yolo"),
        label: String::from("molo nolo"),
        list_id: String::from("han solo"),
        is_done: false,
        details: None
    };
    let todo = TodoItem {
        id: String::from("yolo"),
        label: String::from("dolo"),
        list_id: String::from("han solo"),
        is_done: true,
        details: None
    };

    let vec = vec![todo, todo2];
    let todo_list = TodoList {
        items: vec,
        id: String::from("mjeh"),
        name: String::from("Yolo list")
    };
    
    html! {
        <TodoComp list={todo_list}></TodoComp>
    }
}

async fn list() -> &'static str  {
    "<h1>Test works</h1>"
}
