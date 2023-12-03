use askama::Template;
use axum::{routing::get, Router};

pub fn init(app: Router) -> Router {
    app.route("/", get(index))
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index() -> Index {
    Index {}
}
