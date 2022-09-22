mod basics;

use std::net::SocketAddr;
use axum::{Router, http::StatusCode, Json, routing::get, extract::State};
use basics::introduce;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use tokio_postgres::{NoTls, Row};

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

static DB_INIT_SCRIPT: &'static str = include_str!("../db.sql");

fn internal_error<E>(error: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}

#[derive(Deserialize)]
struct CrabToBe {
    name: String,
    age: i16,
    height: f32
}

#[derive(Serialize)]
struct Crab {
    id: i32,
    name: String,
    age: i16,
    height: f32
}

impl Crab {
    fn from_row(row: Row) -> Crab {
        Crab {
            id: row.get("id"),
            name: row.get("name"),
            age: row.get("age"),
            height: row.get("height")
        }
    }
}

#[tokio::main]
async fn main() {
    // db connection
    let manager = PostgresConnectionManager::new_from_stringlike("host=localhost port=5432 user=postgres password=admin", NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    // initialize db
    init_db(&pool)
        .await
        .expect("Database initialization failed");

    let state = pool;

    // create the routes and inject our connection pool as our state to the requests
    let app = Router::with_state(state)
        .route("/crabs", get(get_crabs).post(add_crab));

    // start the server
    let addr = SocketAddr::from(([127,0,0,1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

/*
    try to run the init script
    causes a panic in the expect when propagating an Err
*/
async fn init_db(pool: &ConnectionPool) -> Result<(), (StatusCode, String)> {
    let conn = pool
        .get()
        .await
        .map_err(internal_error)?;
    
    conn
        .batch_execute(DB_INIT_SCRIPT)
        .await
        .map_err(internal_error)?;

    Ok(())
}

/*
    select all crabs from the db
    map them to a vec of crab objects using our to_row function

    axum handlers take a variable amount of extractors (only the last one can consume the body)
    State extractor extracts the state we added earlier ::with_state(pool)

    TUPLE STRUCTS:
    similar to struct, no named fields. e.g. here struct Extension(T)
    Struct(value): Struct<Type> destructures the struct directly in the function signature
    gives us direct access to values inside extension without the need to access the surrounding struct first
 */
async fn get_crabs(
    State(pool): State<ConnectionPool>
) -> Result<(StatusCode, Json<Vec<Crab>>), (StatusCode, String)> {
    let conn = pool
        .get()
        .await
        .map_err(internal_error)?;

    let result = conn
        .query("SELECT * FROM crabs", &[])
        .await
        .map_err(internal_error)?;

    let crabs = result.into_iter().map(
        |row| Crab::from_row(row)).collect();

    Ok((StatusCode::OK, Json(crabs)))
}

/*
    uses json extractor to deserialize the request body to a CrabToBe (note the derive(Deserialize))
    inserts payload values into the db
*/
async fn add_crab(
    State(pool): State<ConnectionPool>,
    Json(payload): Json<CrabToBe>
) -> Result<StatusCode, (StatusCode, String)> {
    let conn = pool
        .get()
        .await
        .map_err(internal_error)?;
    
    conn
        .execute("INSERT INTO crabs (name, age, height) VALUES ($1, $2, $3)", &[&payload.name, &payload.age, &payload.height])
        .await
        .map_err(internal_error)?;

    Ok(StatusCode::CREATED)
}