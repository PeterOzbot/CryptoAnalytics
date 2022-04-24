use sqlx::PgPool;
use tide::Server;

use crate::database::DbPool;

use super::{endpoints::Endpoints, state::State};

pub struct RestServer {}
impl RestServer {
    pub async fn start() {
        tide::log::start();

        let db_pool = DbPool::make_pool().await;
        let app = RestServer::server(db_pool).await;
        RestServer::listen(app).await;
    }

    async fn listen(app: Server<State>) {
        if let Ok(url) = std::env::var("SERVER_URL") {
            if let Err(err) = app.listen(url).await {
                panic!("Server failed to start: {:}.", err)
            }
        } else {
            panic!("SERVER_URL missing in .env.")
        }
    }

    async fn server(db_pool: PgPool) -> Server<State> {
        let mut app = tide::with_state(State { db_pool: db_pool });

        // CORS
        app.with(
            tide::security::CorsMiddleware::new()
                .allow_methods("GET".parse::<tide::http::headers::HeaderValue>().unwrap())
                .allow_origin(tide::security::Origin::from("*"))
                .allow_credentials(true),
        );

        Endpoints::register(&mut app);
        app
    }
}
