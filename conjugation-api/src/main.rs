use std::{fmt::Display, str::FromStr};

use juniper::{http::graphiql, http::GraphQLRequest, EmptyMutation, EmptySubscription, RootNode};
use lazy_static::lazy_static;
use sqlx::{FromRow, SqlitePool};
use tide::{http::mime, Body, Redirect, Request, Response, Server, StatusCode};

const DB_URL: &str = "sqlite://data/verbs.db";

#[derive(Clone, juniper::GraphQLEnum)]
enum Tense {
    Presente,
    Preterito,
    Imperfecto,
    Futuro,
}

impl Display for Tense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tense::Presente => write!(f, "Presente"),
            Tense::Preterito => write!(f, "Pretérito"),
            Tense::Imperfecto => write!(f, "Imperfecto"),
            Tense::Futuro => write!(f, "Futuro"),
        }
    }
}

impl FromStr for Tense {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Presente" => Ok(Self::Presente),
            "Pretérito" => Ok(Self::Preterito),
            "Imperfecto" => Ok(Self::Imperfecto),
            "Futuro" => Ok(Self::Futuro),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
struct ConjugatedVerb {
    infinitive: String,
    verb_english: String,
    tense: Tense,
    yo: String,
    tu: String,
    el: String,
    nosotros: String,
    vosotros: String,
    ellos: String,
}

impl From<Verb> for ConjugatedVerb {
    fn from(value: Verb) -> Self {
        let Verb {
            infinitive,
            tense,
            verb_english,
            form_1s,
            form_2s,
            form_3s,
            form_1p,
            form_2p,
            form_3p,
        } = value;
        Self {
            infinitive,
            verb_english,
            tense: Tense::from_str(tense.as_str()).unwrap_or(Tense::Presente),
            yo: form_1s,
            tu: form_2s,
            el: form_3s,
            nosotros: form_1p,
            vosotros: form_2p,
            ellos: form_3p,
        }
    }
}

#[derive(Clone, FromRow, Debug)]
struct Verb {
    infinitive: String,
    tense: String,
    verb_english: String,
    form_1s: String,
    form_2s: String,
    form_3s: String,
    form_1p: String,
    form_2p: String,
    form_3p: String,
}

#[juniper::graphql_object]
#[graphql(description = "A Conjugated Verb")]
impl ConjugatedVerb {
    #[graphql(description = "Infinitive form of the verb")]
    fn infinitive(&self) -> &str {
        &self.infinitive
    }

    #[graphql(description = "English form of the verb")]
    fn verb_english(&self) -> &str {
        &self.verb_english
    }

    #[graphql(description = "Tense the verb has been conjugated")]
    fn tense(&self) -> &Tense {
        &self.tense
    }

    #[graphql(description = "First person singular")]
    fn yo(&self) -> &str {
        &self.yo
    }

    #[graphql(description = "Second person singular")]
    fn tu(&self) -> &str {
        &self.tu
    }

    #[graphql(description = "Third person singular")]
    fn el(&self) -> &str {
        &self.el
    }

    #[graphql(description = "First person plural")]
    fn nosotros(&self) -> &str {
        &self.nosotros
    }

    #[graphql(description = "Second person plural")]
    fn vosotros(&self) -> &str {
        &self.vosotros
    }

    #[graphql(description = "Third person plural")]
    fn ellos(&self) -> &str {
        &self.ellos
    }
}

#[derive(Clone)]
pub struct State {
    pool: SqlitePool,
}
impl juniper::Context for State {}

pub struct QueryRoot;

#[juniper::graphql_object(context = State)]
impl QueryRoot {
    #[graphql(description = "get a verb")]
    async fn conjugated_verb(
        context: &State,
        infinitive: String,
        tense: Tense,
    ) -> Option<ConjugatedVerb> {
        sqlx::query_as::<_, Verb>("SELECT infinitive, tense, verb_english, form_1s, form_2s, form_3s, form_1p, form_2p, form_3p FROM verbs WHERE infinitive = ? AND tense = ? AND mood = ?").
            bind(infinitive).
            bind(tense.to_string()).
            bind("Indicativo").
            fetch_optional(&context.pool).
            await.
            unwrap_or_default().
            map(ConjugatedVerb::from)
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<State>, EmptySubscription<State>>;
lazy_static! {
    static ref SCHEMA: Schema =
        Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new());
}

async fn handle_graphql(mut request: Request<State>) -> tide::Result {
    let query: GraphQLRequest = request.body_json().await?;
    let response = query.execute(&SCHEMA, request.state()).await;
    let status = if response.is_ok() {
        StatusCode::Ok
    } else {
        StatusCode::BadRequest
    };

    Ok(Response::builder(status)
        .body(Body::from_json(&response)?)
        .build())
}

async fn handle_graphiql(_: Request<State>) -> tide::Result<impl Into<Response>> {
    Ok(Response::builder(200)
        .body(graphiql::graphiql_source("/graphql", None))
        .content_type(mime::HTML))
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let mut app = Server::with_state(State { pool: db });
    app.with(
        tide::security::CorsMiddleware::new()
            .allow_methods(
                "GET, PUT, POST, DELETE, HEAD, OPTIONS"
                    .parse::<http_types::headers::HeaderValue>()
                    .unwrap(),
            )
            .allow_origin(tide::security::Origin::from("http://localhost:5173"))
            .allow_headers(
                "Authorization, Accept, Content-Type"
                    .parse::<http_types::headers::HeaderValue>()
                    .unwrap(),
            )
            .allow_credentials(true),
    );
    app.at("/").get(Redirect::permanent("/graphiql"));
    app.at("/graphql").post(handle_graphql);
    app.at("/graphiql").get(handle_graphiql);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
