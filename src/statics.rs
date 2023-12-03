use askama_axum::IntoResponse;
use axum::{http::header, routing::get, Router};
use rust_embed::RustEmbed;

pub fn init(app: Router) -> Router {
    app.route("/index.css", get(static_index_css))
        .route(
            "/tailwind_preflight.css",
            get(static_tailwind_preflight_css),
        )
        .route("/htmx.min.js", get(static_htmx_min_js))
}

#[derive(RustEmbed)]
#[folder = "static"]
struct Asset;
async fn static_htmx_min_js() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/javascript")],
        Asset::get("htmx.min.js").unwrap().data.to_vec(),
    )
        .into_response()
}
async fn static_index_css() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css")],
        Asset::get("index.css").unwrap().data.to_vec(),
    )
        .into_response()
}
async fn static_tailwind_preflight_css() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css")],
        Asset::get("tailwind_preflight.css").unwrap().data.to_vec(),
    )
        .into_response()
}
