use super::data::Data;

pub enum Message {
    MakeReq,
    Resp(Result<Data, anyhow::Error>),
}