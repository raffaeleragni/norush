use velvet_web::prelude::*;

pub fn app() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/board", get(board))
        .route("/state", post(state))
        .route("/card", post(card))
        .authorized_cookie_claims("/login", |_: Claims| Ok(AuthResult::OK))
        .route("/register", get(register_form).post(register_do))
        .route("/login", get(login_form).post(login_do))
        .route("/logout", get(logout))
}

#[derive(Deserialize)]
struct Claims {}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate;

async fn login_form() -> impl IntoResponse {
    LoginTemplate
}

async fn register_form() -> impl IntoResponse {
    RegisterTemplate
}

#[derive(Deserialize)]
struct RegisterForm {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn register_do(
    Extension(db): Extension<Pool<Sqlite>>,
    Form(register_form): Form<RegisterForm>,
) -> AppResult<Redirect> {
    let confirmation_code = register_user(
        &db,
        &register_form.username,
        &register_form.email,
        &register_form.password,
    )
    .await?;
    register_user_confirm(&db, &register_form.username, &confirmation_code).await?;
    Ok(Redirect::to("/login"))
}

async fn login_do(
    Extension(db): Extension<Pool<Sqlite>>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> AppResult<(CookieJar, Redirect)> {
    login_cookie(jar, "/", &db, &form.username, &form.password).await
}

async fn logout(jar: CookieJar) -> AppResult<(CookieJar, Redirect)> {
    logout_cookie(jar, "/login")
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[derive(Template, Default)]
#[template(path = "board.html")]
struct Board {
    states: Vec<StateWithCards>,
}

#[derive(Default)]
struct StateWithCards {
    state: &'static str,
    cards: Vec<Card>,
}

impl StateWithCards {
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

#[derive(Template, Default)]
#[template(path = "state.html")]
struct StatePage {
    state: StateWithCards,
}

#[derive(Default)]
struct Card {
    id: u64,
    title: &'static str,
}

async fn index() -> Index {
    Index {}
}

fn statecards_todo() -> StateWithCards {
    StateWithCards {
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
    }
}

fn statecards_inprogress() -> StateWithCards {
    StateWithCards {
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
    }
}

fn statecards_done() -> StateWithCards {
    StateWithCards {
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
    }
}

async fn board() -> Board {
    Board {
        states: vec![
            statecards_todo(),
            statecards_inprogress(),
            statecards_done(),
        ],
    }
}

#[derive(Deserialize)]
struct StateForm {
    state: String,
}

async fn state(Form(state): Form<StateForm>) -> StatePage {
    match state.state.as_str() {
        "todo" => StatePage {
            state: statecards_todo(),
        },
        "in progress" => StatePage {
            state: statecards_inprogress(),
        },
        "done" => StatePage {
            state: statecards_done(),
        },
        _ => StatePage {
            state: StateWithCards::default(),
        },
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
    use super::StateWithCards;

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

    fn state_of(state: &'static str) -> StateWithCards {
        StateWithCards {
            state,
            cards: Vec::new(),
        }
    }
}
