use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "/about"]
    About,
    #[to = "/catalog/{u}/{p}/{s}"]
    Catalog(i32, i32, i32),
    #[to = "/login"]
    Login,
    #[to = "/"]
    Home
}