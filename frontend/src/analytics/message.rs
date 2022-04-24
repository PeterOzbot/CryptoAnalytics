use crate::models::Crypto;

pub enum Message {
    LoadDefinitions,
    DefinitionsLoaded(Result<Vec<Crypto>, anyhow::Error>),
}
