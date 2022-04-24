use yew_router::Switch;

//#[derive(Switch)]
#[derive(Switch, Clone, Debug)]
pub enum ApplicationRoutes {
    #[to = "/portfolio"]
    Portfolio,
    #[to = "/"]
    Home,
}
