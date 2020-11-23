use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{About, Catalog, Home, Login};
use crate::routes::Route;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: Route| match switch {
            Route::About         => html! {<About />},
            Route::Catalog(u, p, s) => html! {<Catalog userid=u primarytype=p subtype=s />},
            Route::Home          => html! {<Home />},
            Route::Login         => html! {<Login />},
        });

        html! {
            <Router<Route, ()> render=render />
        }
    }
}