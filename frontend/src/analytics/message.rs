use crate::models::Crypto;

pub enum Message {
    Refresh,
    LoadDefinitions,
    DefinitionsLoaded(Result<Vec<Crypto>, anyhow::Error>),
}
