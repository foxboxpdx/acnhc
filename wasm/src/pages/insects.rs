use crate::api;
use crate::components::SideBar;
use anyhow::Error;
use yew::format::Json;
use yew::services::fetch::FetchTask;
use yew::prelude::*;
use std::collections::BTreeMap;

struct State {
    data: BTreeMap<String, String>,
    get_data_error: Option<Error>,
    get_data_loaded: bool
}

pub struct Insects {
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>
}

pub enum Msg {
    GetData,
    GetDataSuccess(BTreeMap<String, String>),
    GetDataError(Error)
}

impl Component for Insects {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let data = BTreeMap::new();
        link.send_message(Msg::GetData);
        Self {
            state: State {
                data,
                get_data_error: None,
                get_data_loaded: false
            },
            link: link,
            task: None
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! { "Hello" }
    }
}