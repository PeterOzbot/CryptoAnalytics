use super::cryptos::Cryptos;

pub enum Msg {
    MakeReq,
    Resp(Result<Cryptos, anyhow::Error>),
}