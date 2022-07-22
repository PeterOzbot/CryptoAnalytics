use std::rc::Rc;

use crate::store::CryptoState;

pub enum Message {
    State(Rc<CryptoState>),
}
