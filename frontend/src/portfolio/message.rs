use crate::{common::Error, models::Crypto};

pub enum Message {
    LoadDefinitions,
    DefinitionsLoaded(Result<Vec<Crypto>, Error>),
}
