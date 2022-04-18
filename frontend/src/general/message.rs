use crate::models::ApiData;

pub enum Message {
    MakeReq,
    Resp(Result<ApiData, anyhow::Error>),
}
