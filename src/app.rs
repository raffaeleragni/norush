use askama::Template;
use axum::{routing::get, Router};

pub fn init(app: Router) -> Router {
    app.route("/", get(index))
        .route("/board", get(board))
        .route("/card", get(card))
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[derive(Template, Default)]
#[template(path = "board.html")]
struct Board {
    cards: Vec<Card>,
}

#[derive(Template, Default)]
#[template(path = "card.html")]
struct Card;

async fn index() -> Index {
    Index {}
}

async fn board() -> Board {
    Board {
        cards: vec![Card {}],
    }
}

async fn card() -> Card {
    Card {}
}
