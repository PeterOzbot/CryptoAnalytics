use std::rc::Rc;

use yewdux::prelude::{Store, StoreLink};

use super::CryptoState;

#[derive(Default, Clone)]
pub struct CryptoStore {
    state: Rc<CryptoState>,
}

impl Store for CryptoStore {
    type Model = CryptoState;
    type Message = ();
    type Input = ();
    type Output = ();

    fn new(_link: StoreLink<Self>) -> Self {
        CryptoStore {
            state: Rc::new(CryptoState::new()),
        }
    }

    fn state(&mut self) -> &mut Rc<Self::Model> {
        &mut self.state
    }
}
