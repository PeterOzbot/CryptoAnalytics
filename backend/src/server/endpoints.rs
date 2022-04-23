use tide::{Body, Response, Server};

use crate::database::Provider;

use super::state::State;

pub struct Endpoints {}
impl Endpoints {
    pub fn register(app: &mut Server<State>) {
        app.at("/definitions").get(Endpoints::definitions);
        app.at("/definition/:api_key").get(Endpoints::entries);
    }

    async fn definitions(req: tide::Request<State>) -> tide::Result {
        let definitions = Provider::get_definitions(&req.state().db_pool.clone()).await?;

        let mut res = Response::new(200);
        res.set_body(Body::from_json(&definitions)?);
        Ok(res)
    }

    async fn entries(req: tide::Request<State>) -> tide::Result {
        let definition_id = req.param("api_key")?;

        let entries = Provider::get_entries(definition_id, &req.state().db_pool.clone()).await?;

        let mut res = Response::new(200);
        res.set_body(Body::from_json(&entries)?);
        Ok(res)
    }
}
