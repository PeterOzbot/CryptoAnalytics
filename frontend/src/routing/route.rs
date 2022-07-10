use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum ApplicationRoutes {
    #[at("/")]
    Home,
    #[at("/portfolio")]
    Portfolio,
}
