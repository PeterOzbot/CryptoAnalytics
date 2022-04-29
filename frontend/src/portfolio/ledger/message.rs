use crate::models::Entry;

pub enum Message {
    LoadEntries,
    EntriesLoaded(Result<Vec<Entry>, anyhow::Error>),
}
