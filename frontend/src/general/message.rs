use crate::common::data::Data;

pub enum Message {
    MakeReq,
    Resp(Result<Data, anyhow::Error>),
}
