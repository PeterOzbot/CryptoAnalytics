use server::RestServer;

mod database;
mod models;
mod server;
mod services;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();

    RestServer::start().await;
}
