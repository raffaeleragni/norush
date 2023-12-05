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

impl StateCards {
    fn stripped_state(&self) -> String {
        self.state
            .to_string()
            .replace(&[' ', '-', '_'][..], "")
            .to_lowercase()
    }
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

#[cfg(test)]
mod test {
    use super::StateCards;

    #[test]
    fn stripped_state_empty() {
        let state = state_of("");
        assert_eq!(state.stripped_state(), "");
    }

    #[test]
    fn stripped_state_word() {
        let state = state_of("word");
        assert_eq!(state.stripped_state(), "word");
    }

    #[test]
    fn stripped_state_words() {
        let state = state_of("word word");
        assert_eq!(state.stripped_state(), "wordword");
    }

    #[test]
    fn stripped_state_words_more_spaces() {
        let state = state_of("word  word");
        assert_eq!(state.stripped_state(), "wordword");
    }

    #[test]
    fn stripped_state_symbols() {
        let state = state_of("word-word_word-Word");
        assert_eq!(state.stripped_state(), "wordwordwordword");
    }

    fn state_of(state: &'static str) -> StateCards {
        StateCards {
            state,
            cards: Vec::new(),
        }
    }
}
