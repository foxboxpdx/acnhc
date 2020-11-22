use crate::routes::Route;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub struct HomeMetaCard {
    props: Props
}

#[derive(Properties, Clone)]
pub struct Props {
    pub data: String
}

impl Component for HomeMetaCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {"Hello"}
    }
}