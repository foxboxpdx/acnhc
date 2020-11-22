use crate::routes::Route;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub struct SideBar {}

impl Component for SideBar {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;
        html! {"Hello"}
    }
}