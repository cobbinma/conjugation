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
    Pluscuamperfecto,
    Condicional,
    FuturoPerfecto,
    PreteritoAnterior,
    CondicionalPerfecto,
}

impl Display for Tense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tense::Presente => write!(f, "Presente"),
            Tense::Preterito => write!(f, "Pretérito"),
            Tense::Imperfecto => write!(f, "Imperfecto"),
            Tense::Futuro => write!(f, "Futuro"),
            Tense::PresentePerfecto => write!(f, "Presente perfecto"),
            Tense::Pluscuamperfecto => write!(f, "Pluscuamperfecto"),
            Tense::Condicional => write!(f, "Condicional"),
            Tense::FuturoPerfecto => write!(f, "Futuro perfecto"),
            Tense::PreteritoAnterior => write!(f, "Pretérito anterior"),
            Tense::CondicionalPerfecto => write!(f, "Condicional perfecto"),
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
            "Pluscuamperfecto" => Ok(Self::Pluscuamperfecto),
            "Condicional" => Ok(Self::Condicional),
            "Futuro perfecto" => Ok(Self::FuturoPerfecto),
            "Pretérito anterior" => Ok(Self::PreteritoAnterior),
            "Condicional perfecto" => Ok(Self::CondicionalPerfecto),
            _ => Err(()),
        }
    }
}

#[derive(Clone, juniper::GraphQLEnum)]
enum Mood {
    Indicativo,
    Subjuntivo,
    ImperativoAfirmativo,
    ImperativoNegativo,
}

impl Display for Mood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Indicativo => write!(f, "Indicativo"),
            Self::Subjuntivo => write!(f, "Subjuntivo"),
            Self::ImperativoAfirmativo => write!(f, "Imperativo Afirmativo"),
            Self::ImperativoNegativo => write!(f, "Imperativo Negativo"),
        }
    }
}

impl FromStr for Mood {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Indicativo" => Ok(Self::Indicativo),
            "Subjuntivo" => Ok(Self::Subjuntivo),
            "Imperativo Afirmativo" => Ok(Self::ImperativoAfirmativo),
            "Imperativo Negativo" => Ok(Self::ImperativoNegativo),
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
struct Verb {
    infinitive: String,
    infinitive_english: String,
    gerundio: String,
    gerundio_english: String,
    tenses: Vec<VerbTense>,
}

#[derive(Clone)]
struct VerbTense {
    infinitive: String,
    verb_english: Option<String>,
    tense: Tense,
    mood: Mood,
    conjugations: Vec<Conjugation>,
}

impl From<RepositoryConjugations> for VerbTense {
    fn from(value: RepositoryConjugations) -> Self {
        let RepositoryConjugations {
            infinitive,
            tense,
            mood,
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
            if !spanish.is_empty() {
                conjugations.push(Conjugation {
                    pronoun: Pronoun::Yo,
                    spanish,
                })
            }
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
            if !spanish.is_empty() {
                conjugations.push(Conjugation {
                    pronoun: Pronoun::Nosotros,
                    spanish,
                })
            }
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
            mood: Mood::from_str(mood.as_str()).unwrap_or(Mood::Indicativo),
            conjugations,
        }
    }
}

#[derive(Clone, FromRow, Debug)]
struct RepositoryInfinitive {
    infinitive: String,
    infinitive_english: String,
}

#[derive(Clone, FromRow, Debug)]
struct RepositoryGerund {
    gerund: String,
    gerund_english: String,
}

#[derive(Clone, FromRow, Debug)]
struct RepositoryConjugations {
    infinitive: String,
    tense: String,
    mood: String,
    verb_english: Option<String>,
    form_1s: Option<String>,
    form_2s: Option<String>,
    form_3s: Option<String>,
    form_1p: Option<String>,
    form_2p: Option<String>,
    form_3p: Option<String>,
}

#[juniper::graphql_object]
#[graphql(description = "A Verb")]
impl Verb {
    #[graphql(description = "Infinitive form of the verb")]
    fn infinitive(&self) -> &str {
        &self.infinitive
    }

    #[graphql(description = "English translation of the infinitive")]
    fn infinitive_english(&self) -> &str {
        &self.infinitive_english
    }

    #[graphql(description = "Gerundio")]
    fn gerundio(&self) -> &str {
        &self.gerundio
    }

    #[graphql(description = "English translation of the gerundio form")]
    fn gerundio_english(&self) -> &str {
        &self.gerundio_english
    }

    #[graphql(description = "Tenses")]
    fn tenses(&self) -> &Vec<VerbTense> {
        &self.tenses
    }
}

#[juniper::graphql_object]
#[graphql(description = "A Verb Tense")]
impl VerbTense {
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

    #[graphql(description = "Mood the verb has been conjugated")]
    fn mood(&self) -> &Mood {
        &self.mood
    }

    #[graphql(description = "Title of the combined tense and mood")]
    fn title(&self) -> &str {
        match (&self.tense, &self.mood) {
            (Tense::Presente, Mood::Indicativo) => "Presente de Indicativo",
            (Tense::Imperfecto, Mood::Indicativo) => "Imperfecto de Indicativo",
            (Tense::Preterito, Mood::Indicativo) => "Pretérito",
            (Tense::Futuro, Mood::Indicativo) => "Futuro",
            (Tense::Condicional, Mood::Indicativo) => "Potencial Simple",
            (Tense::Presente, Mood::Subjuntivo) => "Presente de Subjuntivo",
            (Tense::Imperfecto, Mood::Subjuntivo) => "Imperfecto de Subjuntivo",
            (Tense::PresentePerfecto, Mood::Indicativo) => "Perfecto de Indicativo",
            (Tense::Pluscuamperfecto, Mood::Indicativo) => "Pluscuamperfecto de Indicativo",
            (Tense::PreteritoAnterior, Mood::Indicativo) => "Pretérito Anterior",
            (Tense::FuturoPerfecto, Mood::Indicativo) => "Futuro Perfecto",
            (Tense::CondicionalPerfecto, Mood::Indicativo) => "Potencial Compuesto",
            (Tense::PresentePerfecto, Mood::Subjuntivo) => "Perfecto de Subjuntivo",
            (Tense::Pluscuamperfecto, Mood::Subjuntivo) => "Pluscuamperfecto de Subjuntivo",
            (Tense::Presente, Mood::ImperativoAfirmativo) => "Imperativo Afirmativo",
            (Tense::Presente, Mood::ImperativoNegativo) => "Imperativo Negativo",
            _ => "",
        }
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
    async fn verb(context: &State, infinitive: String) -> Option<Verb> {
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "SELECT infinitive, infinitive_english FROM infinitive WHERE infinitive = ",
        );
        query_builder.push_bind(infinitive.clone());

        let query = query_builder.build_query_as::<RepositoryInfinitive>();

        let Some(RepositoryInfinitive {
            infinitive: inf,
            infinitive_english,
        }) = query
            .fetch_optional(&context.pool)
            .await
            .unwrap_or_default()
        else {
            return None;
        };

        let mut query_builder: QueryBuilder<Sqlite> =
            QueryBuilder::new("SELECT gerund, gerund_english FROM gerund WHERE infinitive = ");
        query_builder.push_bind(infinitive.clone());

        let query = query_builder.build_query_as::<RepositoryGerund>();

        let Some(RepositoryGerund {
            gerund,
            gerund_english,
        }) = query
            .fetch_optional(&context.pool)
            .await
            .unwrap_or_default()
        else {
            return None;
        };

        let mut query_builder: QueryBuilder<Sqlite> =
            QueryBuilder::new("SELECT infinitive, tense, mood, verb_english, form_1s, form_2s, form_3s, form_1p, form_2p, form_3p FROM verbs WHERE NOT (mood = 'Subjuntivo' AND (tense = 'Futuro' OR tense = 'Futuro perfecto')) AND infinitive = ");

        query_builder.push_bind(infinitive);

        let query = query_builder.build_query_as::<RepositoryConjugations>();

        let tenses: Vec<VerbTense> = query
            .fetch_all(&context.pool)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(VerbTense::from)
            .collect();

        Some(Verb {
            infinitive: inf,
            infinitive_english,
            gerundio: gerund,
            gerundio_english: gerund_english,
            tenses,
        })
    }

    #[graphql(description = "get a conjugated verb")]
    async fn verb_tense(
        context: &State,
        infinitive: Option<String>,
        tenses: Option<Vec<Tense>>,
    ) -> Option<VerbTense> {
        let mut query_builder: QueryBuilder<Sqlite> =
            QueryBuilder::new("SELECT infinitive, tense, mood, verb_english, form_1s, form_2s, form_3s, form_1p, form_2p, form_3p FROM verbs WHERE  mood = 'Indicativo'");

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

        let query = query_builder.build_query_as::<RepositoryConjugations>();

        query
            .fetch_optional(&context.pool)
            .await
            .unwrap_or_default()
            .map(VerbTense::from)
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
