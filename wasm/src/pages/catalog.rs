use crate::api;
use crate::components::SideBar;
use crate::types::CatalogData;
use anyhow::Error;
use yew::format::Json;
use yew::services::fetch::FetchTask;
use yew::prelude::*;

struct State {
    data: Vec<CatalogData>,
    get_data_error: Option<Error>,
    get_data_loaded: bool
}

#[derive(Properties, Clone)]
pub struct Props {
    pub userid: i32,
    pub primarytype: i32,
    pub subtype: i32
}

pub struct Catalog {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>
}

pub enum Msg {
    GetData,
    GetDataSuccess(Vec<CatalogData>),
    GetDataError(Error)
}

impl Component for Catalog {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let data = Vec::new();
        link.send_message(Msg::GetData);
        Self {
            props,
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
        match message {
            Msg::GetData => {
                self.state.get_data_loaded = false;
                let handler = self.link.callback(
                    move |response: api::FetchResponse<Vec<CatalogData>>| {
                        let (_, Json(data)) = response.into_parts();
                        match data {
                            Ok(data) => Msg::GetDataSuccess(data),
                            Err(err) => Msg::GetDataError(err)
                        }
                    }
                );
                self.task = Some(api::get_catalog_data(self.props.userid, self.props.primarytype, self.props.subtype, handler));
                true
            }
            Msg::GetDataSuccess(data) => {
                self.state.data = data;
                self.state.get_data_loaded = true;
                true
            }
            Msg::GetDataError(error) => {
                self.state.get_data_error = Some(error);
                self.state.get_data_loaded = true;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        if !self.state.get_data_loaded {
            html! {
                <div class="loading_spinner_container">
                    <div class="loading_spinner"></div>
                    <div class="loading_spinner_text">{"Loading..."}</div>
                </div>
            }
        } else if let Some(_) = self.state.get_data_error {
            html! {
                <div>
                    <span>{"Error loading data!"}</span>
                </div>
            }
        } else {
            // format data or whatever
            let title = format!("Catalog Data");
            html! {
                <div class="mainbody">
                    <SideBar/>
                    <div class="title_and_content">
                        <div class="titlebar">
                            <div class="titlebar_text">{title}</div>
                        </div>
                        <div class="content">
                            <div class="catalog_card_list"></div>
                        </div>
                    </div>
                </div>
            }
        }
    }
}