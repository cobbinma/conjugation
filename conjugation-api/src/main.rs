use std::{fmt::Display, str::FromStr};

use juniper::{http::graphiql, http::GraphQLRequest, EmptyMutation, EmptySubscription, RootNode};
use lazy_static::lazy_static;
use sqlx::{FromRow, QueryBuilder, Sqlite, SqlitePool};
use tide::{http::mime, Body, Redirect, Request, Response, Server, StatusCode};

const DB_URL: &str = "sqlite://data/verbs.db";

#[derive(Clone, juniper::GraphQLEnum)]
enum Tense {
    Presente,
    Preterito,
    Imperfecto,
    Futuro,
    PresentePerfecto,
}

impl Display for Tense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tense::Presente => write!(f, "Presente"),
            Tense::Preterito => write!(f, "Pretérito"),
            Tense::Imperfecto => write!(f, "Imperfecto"),
            Tense::Futuro => write!(f, "Futuro"),
            Tense::PresentePerfecto => write!(f, "Presente perfecto"),
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
            "Presente perfecto" => Ok(Self::PresentePerfecto),
            _ => Err(()),
        }
    }
}

#[derive(Clone, juniper::GraphQLEnum)]
enum Pronoun {
    Yo,
    Tu,
    El,
    Nosotros,
    Vosotros,
    Ellos,
}

#[derive(Clone)]
struct Conjugation {
    pronoun: Pronoun,
    spanish: String,
}

#[juniper::graphql_object]
#[graphql(description = "Conjugated forms of a verb")]
impl Conjugation {
    #[graphql(description = "Pronoun used for the conjugation")]
    fn pronoun(&self) -> &Pronoun {
        &self.pronoun
    }

    #[graphql(description = "Conjugated verb in spanish")]
    fn spanish(&self) -> &str {
        &self.spanish
    }
}

#[derive(Clone)]
struct ConjugatedVerb {
    infinitive: String,
    verb_english: Option<String>,
    tense: Tense,
    conjugations: Vec<Conjugation>,
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

        let mut conjugations = vec![];
        if let Some(spanish) = form_1s {
            conjugations.push(Conjugation {
                pronoun: Pronoun::Yo,
                spanish,
            })
        }
        if let Some(spanish) = form_2s {
            conjugations.push(Conjugation {
                pronoun: Pronoun::Tu,
                spanish,
            })
        }
        if let Some(spanish) = form_3s {
            conjugations.push(Conjugation {
                pronoun: Pronoun::El,
                spanish,
            })
        }
        if let Some(spanish) = form_1p {
            conjugations.push(Conjugation {
                pronoun: Pronoun::Nosotros,
                spanish,
            })
        }
        if let Some(spanish) = form_2p {
            conjugations.push(Conjugation {
                pronoun: Pronoun::Vosotros,
                spanish,
            })
        }
        if let Some(spanish) = form_3p {
            conjugations.push(Conjugation {
                pronoun: Pronoun::Ellos,
                spanish,
            })
        }

        Self {
            infinitive,
            verb_english,
            tense: Tense::from_str(tense.as_str()).unwrap_or(Tense::Presente),
            conjugations,
        }
    }
}

#[derive(Clone, FromRow, Debug)]
struct Verb {
    infinitive: String,
    tense: String,
    verb_english: Option<String>,
    form_1s: Option<String>,
    form_2s: Option<String>,
    form_3s: Option<String>,
    form_1p: Option<String>,
    form_2p: Option<String>,
    form_3p: Option<String>,
}

#[juniper::graphql_object]
#[graphql(description = "A Conjugated Verb")]
impl ConjugatedVerb {
    #[graphql(description = "Infinitive form of the verb")]
    fn infinitive(&self) -> &str {
        &self.infinitive
    }

    #[graphql(description = "English form of the verb")]
    fn verb_english(&self) -> &Option<String> {
        &self.verb_english
    }

    #[graphql(description = "Tense the verb has been conjugated")]
    fn tense(&self) -> &Tense {
        &self.tense
    }

    #[graphql(description = "First person singular")]
    fn conjugations(&self) -> &Vec<Conjugation> {
        &self.conjugations
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
    async fn verb(
        context: &State,
        infinitive: Option<String>,
        tenses: Option<Vec<Tense>>,
    ) -> Option<ConjugatedVerb> {
        let mut query_builder: QueryBuilder<Sqlite> =
            QueryBuilder::new("SELECT infinitive, tense, verb_english, form_1s, form_2s, form_3s, form_1p, form_2p, form_3p FROM verbs WHERE  mood = 'Indicativo'");

        if let Some(infinitive) = infinitive {
            query_builder.push(" AND infinitive = ");
            query_builder.push_bind(infinitive);
        };

        let tenses = match tenses {
            Some(tenses) if !tenses.is_empty() => tenses,
            _ => vec![
                Tense::Presente,
                Tense::Imperfecto,
                Tense::Preterito,
                Tense::Futuro,
                Tense::PresentePerfecto,
            ],
        };

        query_builder.push(" AND tense IN (");
        let mut separated = query_builder.separated(", ");
        for tense in tenses.iter() {
            separated.push_bind(tense.to_string());
        }
        separated.push_unseparated(")");

        query_builder.push(" ORDER BY RANDOM() LIMIT 1");

        let query = query_builder.build_query_as::<Verb>();

        query
            .fetch_optional(&context.pool)
            .await
            .unwrap_or_default()
            .map(ConjugatedVerb::from)
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
    app.at("/").get(Redirect::permanent("/graphiql"));
    app.at("/graphql").post(handle_graphql);
    app.at("/graphiql").get(handle_graphiql);
    app.listen("[::]:8080").await?;
    Ok(())
}
