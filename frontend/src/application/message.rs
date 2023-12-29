use super::tab::Tab;
use crate::store::CryptoState;
use std::rc::Rc;

pub enum Message {
    State(Rc<CryptoState>),
    TabChange(Tab),
}
