use sqlx::PgPool;
use tide::{Body, Response, Server};

use crate::{
    database::{DbPool, Provider},
    models::EntryResponse,
};

use super::state::State;

pub struct RestServer {}
impl RestServer {
    pub async fn start() {
        tide::log::start();

        let db_pool = DbPool::make_pool().await;

        let app = server(db_pool).await;
        app.listen("127.0.0.1:8080").await.unwrap();
    }
}

async fn server(db_pool: PgPool) -> Server<State> {
    let state = State { db_pool: db_pool };

    let mut app = tide::with_state(state);

    register_rest_entity(&mut app);

    app
}

fn register_rest_entity(app: &mut Server<State>) {
    app.at("/definitions").get(definitions);
    app.at("/definition/:api_key").get(entries);
}

async fn definitions(req: tide::Request<State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    let definitions = Provider::get_definitions(&db_pool).await?;

    let mut res = Response::new(200);
    res.set_body(Body::from_json(&definitions)?);
    Ok(res)
}

async fn entries(req: tide::Request<State>) -> tide::Result {
    let db_pool = req.state().db_pool.clone();
    let definition_id = req.param("api_key")?;
    let entries = Provider::get_entries(definition_id, &db_pool).await?;

    let mut entries_reponse: Vec<EntryResponse> = vec![];
    for entry in entries {
        entries_reponse.push(EntryResponse {
            id: entry.id,
            definition_id: entry.definition_id,
            date_time: entry.date_time,
            amount: entry.amount.to_string(),
            withdraw_fee: entry.withdraw_fee.to_string(),
            price: entry.price.to_string(),
            purchase_fee: entry.purchase_fee.to_string(),
        });
    }

    let mut res = Response::new(200);
    res.set_body(Body::from_json(&entries_reponse)?);
    Ok(res)
}
