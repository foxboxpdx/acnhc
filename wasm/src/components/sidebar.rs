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
        html! {
            <div class="sidebar">
                <img class="logo" src={"/images/logo.png"}/>
                <div class="sidebar_card_list">
                    <div class="sidebar_category_container">
                        <div class="sidebar_category_title">{"General"}</div>
                        <div class="sidebar_category_list">
                            <div class="sidebar_card_container">
                                <Anchor route=Route::Home classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Home"}</div>
                                </Anchor>
                            </div>
                            <div class="sidebar_card_container">
                                <Anchor route=Route::About classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"About"}</div>
                                </Anchor>
                            </div>
                            <div class="sidebar_card_container">
                                <Anchor route=Route::Login classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Logout"}</div>
                                </Anchor>
                            </div>
                        </div>
                    </div>
                    <div class="sidebar_category_container">
                        <div class="sidebar_category_title">{"Creatures"}</div>
                        <div class="sidebar_category_list">
                            <div class="sidebar_card_container">
                                <Anchor route=Route::Home classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Fish"}</div>
                                </Anchor>
                            </div>
                            <div class="sidebar_card_container">
                                <Anchor route=Route::About classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Insects"}</div>
                                </Anchor>
                            </div>
                            <div class="sidebar_card_container">
                                <Anchor route=Route::Login classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Deep Sea"}</div>
                                </Anchor>
                            </div>
                        </div>
                    </div>
                    <div class="sidebar_category_container">
                        <div class="sidebar_category_title">{"Collectables"}</div>
                        <div class="sidebar_category_list">
                            <div class="sidebar_card_container">
                                <Anchor route=Route::Home classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Fossils"}</div>
                                </Anchor>
                            </div>
                            <div class="sidebar_card_container">
                                <Anchor route=Route::About classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Music"}</div>
                                </Anchor>
                            </div>
                            <div class="sidebar_card_container">
                                <Anchor route=Route::Login classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Artwork"}</div>
                                </Anchor>
                            </div>
                        </div>
                    </div>
                    <div class="sidebar_category_container">
                        <div class="sidebar_category_title">{"Recipes"}</div>
                        <div class="sidebar_category_list">
                            <div class="sidebar_card_container">
                                <Anchor route=Route::Home classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Equipment"}</div>
                                </Anchor>
                            </div>
                            <div class="sidebar_card_container">
                                <Anchor route=Route::About classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Floors"}</div>
                                </Anchor>
                            </div>
                            <div class="sidebar_card_container">
                                <Anchor route=Route::Login classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Housewares"}</div>
                                </Anchor>
                            </div>
                            <div class="sidebar_card_container">
                                <Anchor route=Route::Home classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Miscellaneous"}</div>
                                </Anchor>
                            </div>
                            <div class="sidebar_card_container">
                                <Anchor route=Route::About classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Other"}</div>
                                </Anchor>
                            </div>
                            <div class="sidebar_card_container">
                                <Anchor route=Route::Login classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Rugs"}</div>
                                </Anchor>
                            </div>    
                            <div class="sidebar_card_container">
                                <Anchor route=Route::Home classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Tools"}</div>
                                </Anchor>
                            </div>
                            <div class="sidebar_card_container">
                                <Anchor route=Route::About classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Wall-Mounted"}</div>
                                </Anchor>
                            </div>
                            <div class="sidebar_card_container">
                                <Anchor route=Route::Login classes="sidebar_card_anchor">
                                    <div class="sidebar_menuitem">{"Wallpaper"}</div>
                                </Anchor>
                            </div>                                                    
                        </div>
                    </div>
                </div>
            </div>                                                         
        }
    }
}