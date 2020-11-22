use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "/about"]
    About,
    #[to = "/artwork"]
    Artwork,
    #[to = "/deepsea"]
    DeepSea,
    #[to = "/equipment"]
    Equipment,
    #[to = "/fish"]
    Fish,
    #[to = "/floors"]
    Floors,
    #[to = "/fossils"]
    Fossils,
    #[to = "/housewares"]
    Housewares,
    #[to = "/insects"]
    Insects,
    #[to = "/login"]
    Login,
    #[to = "/miscellaneous"]
    Miscellaneous,
    #[to = "/music"]
    Music,
    #[to = "/other"]
    Other,
    #[to = "/recipes"]
    Recipes,
    #[to = "/rugs"]
    Rugs,
    #[to = "/tools"]
    Tools,
    #[to = "/wallmounted"]
    WallMounted,
    #[to = "/wallpaper"]
    Wallpaper,
    #[to = "/"]
    Home
}