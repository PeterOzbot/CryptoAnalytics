use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Definition {
    pub api_key: String,
    pub precision: i16,
}
