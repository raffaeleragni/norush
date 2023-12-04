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
    states: Vec<StateCards>,
}

#[derive(Default)]
struct StateCards {
    state: &'static str,
    cards: Vec<Card>,
}

#[derive(Template, Default)]
#[template(path = "card.html")]
struct CardPage {
    card: Card,
}

#[derive(Default)]
struct Card {
    title: &'static str,
}

async fn index() -> Index {
    Index {}
}

async fn board() -> Board {
    Board {
        states: vec![
            StateCards {
                state: "todo",
                cards: vec![
                    Card { title: "user actions" },
                    Card { title: "connect to database" },
                ],
            },
            StateCards {
                state: "in progress",
                cards: vec![
                    Card { title: "mock data" },
                    Card { title: "refactor" },
                    Card {
                        title: "setup layout",
                    },
                ],
            },
            StateCards {
                state: "done",
                cards: vec![
                    Card {
                        title: "setup project",
                    },
                    Card {
                        title: "setup build tools",
                    },
                    Card {
                        title: "create github",
                    },
                    Card {
                        title: "announce project",
                    },
                ],
            },
        ],
    }
}

async fn card() -> CardPage {
    CardPage {
        card: Card {
            title: "sample title",
        },
    }
}
