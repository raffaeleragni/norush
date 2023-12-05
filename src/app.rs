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
    id: u64,
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
                    Card {
                        id: 1,
                        title: "user actions",
                    },
                    Card {
                        id: 2,
                        title: "connect to database",
                    },
                ],
            },
            StateCards {
                state: "in progress",
                cards: vec![
                    Card {
                        id: 3,
                        title: "mock data",
                    },
                    Card {
                        id: 4,
                        title: "refactor",
                    },
                    Card {
                        id: 5,
                        title: "setup layout",
                    },
                ],
            },
            StateCards {
                state: "done",
                cards: vec![
                    Card {
                        id: 6,
                        title: "setup project",
                    },
                    Card {
                        id: 7,
                        title: "setup build tools",
                    },
                    Card {
                        id: 8,
                        title: "create github",
                    },
                    Card {
                        id: 9,
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
            id: 0,
            title: "sample title",
        },
    }
}
