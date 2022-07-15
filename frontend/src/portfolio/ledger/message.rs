use crate::{common::Error, models::Entry};

pub enum Message {
    LoadEntries,
    EntriesLoaded(Result<Vec<Entry>, Error>),
}
