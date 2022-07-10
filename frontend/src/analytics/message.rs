use crate::{common::Error, models::Crypto};

pub enum Message {
    LoadDefinitions,
    DefinitionsLoaded(Option<Vec<Crypto>>, Option<Error>),
}
