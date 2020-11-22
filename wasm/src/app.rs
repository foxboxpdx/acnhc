use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::*;
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
            Route::Artwork       => html! {<Artwork />},
            Route::DeepSea       => html! {<DeepSea />},
            Route::Equipment     => html! {<Equipment />},
            Route::Fish          => html! {<Fish />},
            Route::Floors        => html! {<Floors />},
            Route::Fossils       => html! {<Fossils />},
            Route::Home          => html! {<Home />},
            Route::Housewares    => html! {<Housewares />},
            Route::Insects       => html! {<Insects />},
            Route::Login         => html! {<Login />},
            Route::Miscellaneous => html! {<Miscellaneous />},
            Route::Music         => html! {<Music />},
            Route::Other         => html! {<Other />},
            Route::Recipes       => html! {<Recipes />},
            Route::Rugs          => html! {<Rugs />},
            Route::Tools         => html! {<Tools />},
            Route::WallMounted   => html! {<WallMounted />},
            Route::Wallpaper     => html! {<Wallpaper />}
        });

        html! {
            <Router<Route, ()> render=render />
        }
    }
}