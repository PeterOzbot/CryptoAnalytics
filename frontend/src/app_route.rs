extern crate serde;
use yew_router::{Switch};

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/todo/{id}"]
    Detail(i32),
    #[to = "/"]
    Home,
}