use server::RestServer;

mod database;
mod models;
mod server;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();

    RestServer::start().await;
}
