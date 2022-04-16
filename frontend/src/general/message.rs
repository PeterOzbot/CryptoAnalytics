use crate::common::Data;

pub enum Message {
    MakeReq,
    Resp(Result<Data, anyhow::Error>),
}
